import { createServerData$ } from "solid-start/server";
import { useRouteData } from "solid-start";
import { Suspense } from "solid-js";
import Avatar from "~/components/Avatar";
import Externals from "~/components/External";
import Roster from "~/components/Roster";
import { getRoster } from "utils/db";

export function routeData() {
    return createServerData$(
        async () => {
            const roster = await getRoster();
            return roster;
        },
        { key: () => ["roster"], initialValue: [] },
    );
}

export default function Home() {
    const characters = useRouteData<typeof routeData>();
    return (
        <main class="container max-w-2xl m-auto min-h-screen">
            <div class="min-h-[70vh]">
                <div class="flex flex-col gap-2 w-full justify-center items-center">
                    <Avatar style="h-32 w-32" src="/dwarf-icon.webp" />
                    <div class="flex gap-2 justify-center items-center">
                        <img src="/alliance.png" alt="alliance" class="h-10" />
                        <h1 class="text font-bold m-auto text-2xl text-center">
                            Dwarf Invasion
                        </h1>
                    </div>
                    <p>
                        <span class="text-alliance-base"> Alliance</span>,
                        Zul'Jin, US{" "}
                    </p>
                    <p> Tuesday/Wednesday 7:30 - 10:30pm EST </p>
                </div>
                <Externals />
                <Suspense fallback={<div>Loading...</div>}>
                    <Roster characters={characters()!} />
                </Suspense>
            </div>
        </main>
    );
}
