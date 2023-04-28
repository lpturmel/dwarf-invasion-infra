import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { HttpApi } from "@aws-cdk/aws-apigatewayv2-alpha";
import { HttpLambdaIntegration } from "@aws-cdk/aws-apigatewayv2-integrations-alpha";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as targets from "aws-cdk-lib/aws-events-targets";
import * as events from "aws-cdk-lib/aws-events";
import * as iam from "aws-cdk-lib/aws-iam";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as secrets from "aws-cdk-lib/aws-secretsmanager";
import * as dynamo from "aws-cdk-lib/aws-dynamodb";
import { getAssetPath } from "../utils";
import { config } from "dotenv";
import { AttributeType } from "aws-cdk-lib/aws-dynamodb";
config();

export class InfraStack extends cdk.Stack {
    aggregator: lambda.Function;
    creator: lambda.Function;
    webhook: lambda.Function;

    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        const SIM_CREATION_DELAY = 300;
        const REPORT_POLLER_INTERVAL = 600;

        const backendUser = new iam.User(this, "BackendUser", {
            userName: "dwarf-invasion-backend-user",
        });

        const prefix = "dwarfinvasion";
        const role = new iam.Role(this, `${prefix}-eventbridge-role`, {
            assumedBy: new iam.ServicePrincipal("scheduler.amazonaws.com"),
        });

        const bucket = new s3.Bucket(this, `${prefix}-bucket`, {
            bucketName: "dwarf-invasion",
            removalPolicy: cdk.RemovalPolicy.DESTROY,
            publicReadAccess: true,
        });

        const aggregatorHandler = new lambda.Function(
            this,
            `${prefix}-sim-aggregator-handler`,
            {
                functionName: `${prefix}-sim-aggregator-handler`,
                runtime: lambda.Runtime.NODEJS_18_X,
                handler: "index.handler",
                memorySize: 1024,
                code: lambda.Code.fromAsset(
                    getAssetPath("simAggregator", "node"),
                ),
                architecture: lambda.Architecture.ARM_64,
                timeout: cdk.Duration.seconds(30),
                environment: {
                    BUCKET_URL: bucket.urlForObject(),
                    BUCKET_NAME: bucket.bucketName,
                    DISCORD_WH_URL: process.env.DISCORD_WH_URL!,
                    DISCORD_GUILD_ID: process.env.DISCORD_GUILD_ID!,
                    DISCORD_BOT_TOKEN: process.env.DISCORD_BOT_TOKEN!,
                    RAIDBOTS_COOKIE: process.env.RAIDBOTS_COOKIE!,
                },
                description:
                    "This is the handler for aggregating the results of the sims and posting them",
            },
        );
        role.addToPolicy(
            new iam.PolicyStatement({
                actions: ["lambda:InvokeFunction"],
                resources: [aggregatorHandler.functionArn],
            }),
        );

        const simCreatorHandler = new lambda.Function(
            this,
            `${prefix}-sim-creator-handler`,
            {
                functionName: `${prefix}-sim-creator-handler`,
                runtime: lambda.Runtime.NODEJS_18_X,
                handler: "index.handler",
                memorySize: 1024,
                code: lambda.Code.fromAsset(getAssetPath("simCreator", "node")),
                architecture: lambda.Architecture.ARM_64,
                timeout: cdk.Duration.minutes(2),
                environment: {
                    SIM_AGGREGATOR_ARN: aggregatorHandler.functionArn,
                    ROSTER_URL: bucket.urlForObject("roster.json"),
                    EVENT_ROLE_ARN: role.roleArn,
                    DISCORD_GUILD_ID: process.env.DISCORD_GUILD_ID!,
                    DISCORD_BOT_TOKEN: process.env.DISCORD_BOT_TOKEN!,
                    BUCKET_URL: bucket.urlForObject(),
                    SIM_SCHEDULE_DELAY: SIM_CREATION_DELAY.toString(),
                    RAIDBOTS_COOKIE: process.env.RAIDBOTS_COOKIE!,
                },
                description:
                    "This is the handler for initating sims against the raidbot api and create the scheduler",
            },
        );

        // Add policy to allow the lambda to control eventbridge
        simCreatorHandler.addToRolePolicy(
            new iam.PolicyStatement({
                actions: ["scheduler:CreateSchedule", "iam:PassRole"],
                resources: ["*"],
            }),
        );

        const table = new dynamo.Table(this, `${prefix}-table`, {
            tableName: prefix,
            partitionKey: {
                name: "name",
                type: AttributeType.STRING,
            },
        });
        table.grantReadWriteData(backendUser);
        const raiderIoCacheHandler = new lambda.Function(
            this,
            `${prefix}-raiderIo-cache-handler`,
            {
                functionName: `${prefix}-raiderIo-cache-handler`,
                runtime: lambda.Runtime.PROVIDED_AL2,
                handler: "not.required",
                memorySize: 1024,
                timeout: cdk.Duration.minutes(2),
                code: lambda.Code.fromAsset(
                    getAssetPath("raiderio_cache", "rust"),
                ),
                architecture: lambda.Architecture.ARM_64,
                environment: {
                    RUST_BACKTRACE: "1",
                    TABLE_NAME: table.tableName,
                    BUCKET_NAME: bucket.bucketName,
                },
                description:
                    "Raider IO Api data cache (caches characters part of the roster)",
            },
        );

        table.grantReadWriteData(raiderIoCacheHandler);
        const reportsHandler = new lambda.Function(
            this,
            `${prefix}-wl-reports-handler`,
            {
                functionName: `${prefix}-wl-reports-handler`,
                runtime: lambda.Runtime.PROVIDED_AL2,
                handler: "not.required",
                memorySize: 1024,
                code: lambda.Code.fromAsset(getAssetPath("log_finder", "rust")),
                architecture: lambda.Architecture.ARM_64,
                environment: {
                    RUST_BACKTRACE: "1",
                    DISCORD_APP_ID: "1044438772042956861",
                    DISCORD_GUILD_ID: process.env.DISCORD_GUILD_ID!,
                    DISCORD_BOT_TOKEN: process.env.DISCORD_BOT_TOKEN!,
                    WARCRAFT_LOGS_API_KEY: process.env.WARCRAFT_LOGS_API_KEY!,
                    DISCORD_WEBHOOK_URL: process.env.LOGS_DISCORD_WEBHOOK_URL!,
                    POLL_INTERVAL: REPORT_POLLER_INTERVAL.toString(),
                },
                description: "Warcraft log reports poller",
            },
        );
        const webhookHandler = new lambda.Function(
            this,
            `${prefix}-api-handler`,
            {
                functionName: `${prefix}-backend-handler`,
                runtime: lambda.Runtime.PROVIDED_AL2,
                handler: "not.required",
                memorySize: 1024,
                code: lambda.Code.fromAsset(getAssetPath("webhook", "rust")),
                architecture: lambda.Architecture.ARM_64,
                environment: {
                    RUST_BACKTRACE: "1",
                    DISCORD_APP_ID: "1044438772042956861",
                    DISCORD_GUILD_ID: process.env.DISCORD_GUILD_ID!,
                    DISCORD_BOT_TOKEN: process.env.DISCORD_BOT_TOKEN!,
                    BUCKET_URL: bucket.urlForObject(),
                    BUCKET_NAME: bucket.bucketName,
                    SIM_SCHEDULE_DELAY: SIM_CREATION_DELAY.toString(),
                    SIM_CREATOR_FUNCTION_NAME: simCreatorHandler.functionName,
                },
                description:
                    "Bot for Dwarf Invasion Discord server Slash command integration",
            },
        );
        webhookHandler.addToRolePolicy(
            new iam.PolicyStatement({
                actions: ["lambda:InvokeFunction"],
                resources: [simCreatorHandler.functionArn],
            }),
        );

        const raiderIoCacheSchedule = new events.Rule(
            this,
            `${prefix}-raiderio-schedule`,
            {
                schedule: events.Schedule.expression("cron(0 * * * ? *)"),
                targets: [new targets.LambdaFunction(raiderIoCacheHandler)],
            },
        );
        const simSchedule = new events.Rule(this, `${prefix}-schedule`, {
            // Every friday at 5 am EST
            schedule: events.Schedule.expression("cron(0 10 ? * FRI *)"),
            enabled: false,
            targets: [new targets.LambdaFunction(simCreatorHandler)],
        });
        const logPollerSchedule = new events.Rule(
            this,
            `${prefix}-log-poller-schedule`,
            {
                // every 10 minutes
                schedule: events.Schedule.expression("cron(*/10 * * * ? *)"),
                targets: [new targets.LambdaFunction(reportsHandler)],
            },
        );

        // Add lambda permissions to write to S3
        bucket.grantWrite(webhookHandler);
        bucket.grantWrite(aggregatorHandler);

        bucket.grantRead(raiderIoCacheHandler);
        bucket.grantRead(webhookHandler);
        bucket.grantRead(aggregatorHandler);

        const api = new HttpApi(this, `${prefix}-http-api`, {
            apiName: `${prefix}-backend-api`,
            description:
                "Bot for Dwarf Invasion Discord server Slash command integration",
            defaultIntegration: new HttpLambdaIntegration(
                "DwarfInvasionBotApiIntegration",
                webhookHandler,
            ),
        });
        const xav_user = iam.User.fromUserName(
            this,
            `${prefix}-xav-user`,
            "xavier",
        );
        const user_contributor_policy = new iam.Policy(
            this,
            `${prefix}-user-contributor-policy`,
            {
                policyName: `${prefix}-user-contributor-policy`,
                statements: [
                    new iam.PolicyStatement({
                        actions: ["s3:*"],
                        resources: [bucket.bucketArn, bucket.bucketArn + "/*"],
                    }),
                ],
            },
        );
        user_contributor_policy.attachToUser(xav_user);

        let secret_config = secrets.Secret.fromSecretNameV2(
            this,
            `${prefix}-secret-config`,
            "dwarf-invasion",
        );

        secret_config.grantRead(reportsHandler);
        secret_config.grantRead(simCreatorHandler);
        secret_config.grantRead(webhookHandler);
        secret_config.grantRead(aggregatorHandler);
        secret_config.grantRead(raiderIoCacheHandler);

        this.aggregator = aggregatorHandler;
        this.creator = simCreatorHandler;
        this.webhook = webhookHandler;
        // implement the role from eventbridge to invoke the lambda
        new cdk.CfnOutput(this, `${prefix}-api-url`, {
            value: api.url!,
            description: "URL for the API",
        });
        new cdk.CfnOutput(this, `${prefix}-sim-creator-handler-role`, {
            value: simCreatorHandler.role?.roleArn!,
            description: "Role for the sim creator handler",
        });
        new cdk.CfnOutput(this, `${prefix}-rule-role`, {
            value: role.roleArn,
            description:
                "Role for the eventbridge rules created by the sim creator handler",
        });
    }
}
