import { RouteDataFunc } from "@solidjs/router";
import { Show, Suspense } from "solid-js";
import { useParams, useRouteData } from "solid-start";
import { createServerData$, redirect } from "solid-start/server";
import { wowClassMap } from "utils/data";
import { getCharacter } from "utils/db";

export const routeData: RouteDataFunc = (props) => {
    const { character_name } = props.params;
    console.log(character_name);
    return createServerData$(
        async ([, paramName]) => {
            const charInfo = await getCharacter(paramName);

            if (!charInfo) {
                return redirect("/members/notfound");
            }
            return charInfo;
        },
        { key: () => ["character", character_name] },
    );
};

function getDateDifference(date: Date) {
    // get the time difference in seconds, minutes, days, weeks

    const now = new Date();
    const diff = now.getTime() - date.getTime();

    const seconds = Math.floor(diff / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);
    const weeks = Math.floor(days / 7);

    if (seconds < 60) {
        return seconds + " second(s) ago";
    }
    if (minutes < 60) {
        return minutes + " minute(s) ago";
    }
    if (hours < 24) {
        return hours + " hour(s) ago";
    }
    if (days < 7) {
        return days + " day(s) ago";
    }
    return weeks + " week(s) ago";
}
export default function MemberPage() {
    const characterData = useRouteData<typeof routeData>();
    const { characterName } = useParams();
    return (
        <div class="dark:bg-base-300 bg-gray-300 max-w-xl p-8 rounded-md mx-auto mt-40">
            <Suspense fallback={<p> Loading... </p>}>
                <Show when={characterData()}>
                    <div class="flex flex-col gap-4">
                        <div class="flex gap-8 items-center justify-between">
                            <div class="flex flex-col items-start">
                            <div class="flex gap-8 items-center">
                            <img
                                class="rounded-full w-28 h-28 border-2 dark:border-gray-600 border-gray-300"
                                src={characterData().thumbnail_url}
                                alt={characterData().name}
                            />
                            <div class="flex flex-col items-start">
                            <h1
                                class="text-4xl font-3xl font-semibold"
                            >
                                {characterData()!.name}
                            </h1>
                            <p>
                                {characterData().gear.item_level_equipped} ilvl
                            </p>
                            </div>
                            </div>
                            </div>
                            <p
                            class=" p-4 rounded-md font-semibold"
                                style={{
                                    color: wowClassMap[characterData().class]
                                        .color,
                                }}
                            > {characterData().active_spec_name} {characterData().class} </p>
                        </div>
                            <div
                                class="tooltip text-slate-600"
                                data-tip={new Date(
                                    characterData().last_crawled_at,
                                ).toUTCString()}
                            >
                                <p>
                                    {" "}
                                    Last updated:{" "}
                                    {getDateDifference(
                                        new Date(
                                            characterData().last_crawled_at,
                                        ),
                                    )}{" "}
                                </p>
                            </div>
                        <div class="divider" />

                        <div class="flex flex-col">
                            <h1 class="text-3xl font-semibold">Raid</h1>

                            <div class="flex gap-4 justify-between p-4">
                                <p> Vault of the Incarnates </p>
                                <p class="font-semibold">
                                    {
                                        characterData().raid_progression[
                                            "vault-of-the-incarnates"
                                        ].summary
                                    }
                                </p>
                            </div>
                        </div>
                        <div class="divider" />
                        <div class="flex justify-between items-center">
                            <h1 class="text-3xl font-semibold">M+</h1>
                            <p
                                class="font-semibold text-lg"
                                style={{
                                    color: characterData()
                                        .mythic_plus_scores_by_season[0]
                                        .segments.all.color,
                                }}
                            >
                                {" "}
                                {
                                    characterData()
                                        .mythic_plus_scores_by_season[0].scores
                                        .all
                                }{" "}
                            </p>
                        </div>
                    </div>
                </Show>
            </Suspense>
        </div>
    );
}
