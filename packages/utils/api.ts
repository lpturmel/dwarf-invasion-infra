import { RosterItem } from "./types/roster";
import fetch from "node-fetch";

import { CharacterResponse } from "./responses";

export interface BasicPlayerOptions {
    realm: string;
    name: string;
    region: string;
}

export const getRaiderIOClient = () => {
    const BASE_API_URL = "https://raider.io/api/v1";
    return {
        get: async function <T>(path: string) {
            const url = `${BASE_API_URL}${path}`;
            const response = await fetch(url, {
                method: "GET",
                headers: {
                    Accept: "application/json",
                },
            });

            if (response.status === 200) {
                const json = await response.json();

                return json as T;
            }

            return null;
        },
    };
};

export const getBasicPlayer = async (opts: BasicPlayerOptions) => {
    const api = getRaiderIOClient();
    const encodedName = encodeURIComponent(opts.name);
    const data = await api.get<CharacterResponse>(
        `/characters/profile?region=${opts.region}&realm=${opts.realm}&name=${encodedName}`,
    );
    return data!;
};

export const getRoster = async () => {
    const url = "https://s3.us-east-1.amazonaws.com/dwarf-invasion";
    const response = await fetch(`${url}/roster.json`);
    const roster = await response.json();
    return roster as RosterItem[];
};

export function batchPromisesWithDelay<T>(
    promises: Promise<T>[],
    delay: number,
): Promise<T[]> {
    // Create a new array to store the resolved values of the promises
    const resolvedValues: T[] = [];

    // Create a new promise that will resolve when all of the promises in the input array have resolved
    return new Promise((resolve, reject) => {
        // Iterate over the input array of promises
        promises.forEach((promise, index) => {
            // Wait for the specified delay before resolving the promise
            setTimeout(() => {
                // Resolve the promise and store its resolved value
                promise
                    .then((resolvedValue) => {
                        resolvedValues[index] = resolvedValue;

                        // If all of the promises in the input array have been resolved,
                        // resolve the promise returned by this function with the resolved values
                        if (resolvedValues.length === promises.length) {
                            resolve(resolvedValues);
                        }
                    })
                    .catch(reject);
            }, delay * index);
        });
    });
}
