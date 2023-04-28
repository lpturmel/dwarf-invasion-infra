import { Component, For } from "solid-js";
import { A } from "solid-start";
import { wowClassMap } from "utils/data";

interface RosterProps {
    characters: any[];
}
const Roster: Component<RosterProps> = (props) => {
    return (
        <div class="flex min-h-[40vh] flex-col gap-4">
            <p class="text-2xl font-bold"> Roster </p>
            <div class="flex w-full flex-wrap justify-center gap-2">
                <For each={props.characters}>
                    {(character) => (
                        <A href={`/members/profile/${character.name}`}>
                            <img
                                src={character.thumbnail_url}
                                alt={character.name}
                            />
                            <p
                                class="text-sm"
                                style={{
                                    color: wowClassMap[character.class]
                                        .color,
                                }}
                            >
                                {character.name}
                            </p>
                        </A>
                    )}
                </For>
            </div>
        </div>
    );
};

export default Roster;
