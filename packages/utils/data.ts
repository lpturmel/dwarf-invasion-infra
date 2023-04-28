export interface Character {
    name: string;
    realm: string;
    region: string;
    thumbnail: string;
    class: WowClass;
    role: WowRole;
}
export type WowRole = "dps" | "healer" | "tank";
export type WowClass =
    | "druid"
    | "dh"
    | "monk"
    | "rogue"
    | "dk"
    | "mage"
    | "warrior"
    | "warlock"
    | "shaman"
    | "priest"
    | "hunter"
    | "paladin"
    | "evoker";

export const wowClassMap = {
    "Demon Hunter": {
        displayName: "Demon Hunter",
        color: "#A330C9",
    },
    Druid: {
        displayName: "Druid",
        color: "#FF7C0A",
    },
    "Death Knight": {
        displayName: "Death Knight",
        color: "#C41E3A",
    },
    Evoker: {
        displayName: "Evoker",
        color: "#33937F",
    },
    Hunter: {
        displayName: "Hunter",
        color: "#AAD372",
    },
    Mage: {
        displayName: "Mage",
        color: "#3FC7EB",
    },
    Monk: {
        displayName: "Monk",
        color: "#00FF98",
    },
    Paladin: {
        displayName: "Paladin",
        color: "#F48CBA",
    },
    Priest: {
        displayName: "Priest",
        color: "#FFFFFF",
    },
    Rogue: {
        displayName: "Rogue",
        color: "#FFF468",
    },
    Shaman: {
        displayName: "Shaman",
        color: "#0070DD",
    },
    Warlock: {
        displayName: "Warlock",
        color: "#8788EE",
    },
    Warrior: {
        displayName: "Warrior",
        color: "#C69B6D",
    },
};
