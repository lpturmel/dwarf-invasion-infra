import { DynamoDBClient } from "@aws-sdk/client-dynamodb";
import { DynamoDBDocumentClient, GetCommand, ScanCommand} from "@aws-sdk/lib-dynamodb";

const client = new DynamoDBClient({
    region: "us-east-1",
    credentials: {
        accessKeyId: process.env.ACCESS_KEY_ID!,
        secretAccessKey: process.env.SECRET_ACCESS_KEY!,
    },
});
const db = DynamoDBDocumentClient.from(client);

export async function getRoster(
) {
    const req = new ScanCommand({
        TableName: "dwarfinvasion",
    });

    const res = await db.send(req);

    return res.Items as any[];
}
export async function getCharacter(
    characterName: string,
) {
    const req = new GetCommand({
        TableName: "dwarfinvasion",
        Key: {
            name: characterName,
        },
    });
    let res = await db.send(req);
    //TODO
    return res.Item as any;
}
