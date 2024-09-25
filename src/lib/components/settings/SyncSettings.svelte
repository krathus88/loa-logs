<script lang="ts">
    import { settings } from "$lib/utils/settings";
    import SettingItem from "./SettingItem.svelte";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";
    import { uploadLog } from "$lib/utils/sync";

    function saveExcludedCharacters() {
        $settings.sync.excludedCharacters = excludedCharacters.split("\n");
    }

    let excludedCharacters = "";
    onMount(() => {
        excludedCharacters = $settings.sync.excludedCharacters.join("\n");
    });

    let syncing = false;
    let synced = 0;
    let message = "";

    async function syncPastLogs() {
        if (!$settings.sync.enabled) {
            message = "Sync is not enabled.";
            return;
        }

        if (!$settings.sync.auto) {
            message = "Auto upload is not enabled.";
            return;
        }

        if ($settings.sync.accessToken === "") {
            message = "Access token is not set.";
            return;
        }

        if (syncing) {
            return;
        }
        syncing = true;
        synced = 0;
        const ids = await invoke("get_sync_candidates", {});

        const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
        const DELAY_TIME = 2500; // Milliseconds -- Delay Time between mass upload requests
        const BATCH_SIZE = 40; // Maximum number of encounters per batch
        let log_counter = 0;
        let encounters: any[] = []; // Temporary array to hold encounters
        let ids_batch: number[] = [];
        for (let i = 0; i < ids.length; i++) {
            let id = ids[i];
            const encounter = await invoke("load_encounter_sync", { id: id.toString() });

            encounters.push(encounter);
            ids_batch.push(id);

            // If the batch size reaches the limit, process it
            if (encounters.length >= BATCH_SIZE) {
                const encountersInfo = await uploadLog(encounters, ids_batch, $settings.sync);

                // Clear for the next set of encounters
                encounters = [];
                ids_batch = [];

                // If encountersInfo returns data instead of null
                if (encountersInfo) {
                    for (const key in encountersInfo) {
                        log_counter++;

                        const upstream = encountersInfo[key].id;
                        const is_success = encountersInfo[key].success;
                        const is_valid = encountersInfo[key].is_valid;
                        if (is_success === null || is_valid === null) {
                            break;
                        }
                        if (is_success === true && is_valid === true) {
                            synced++;

                            await invoke("write_log", {
                                message: "Uploaded encounter " + key + " upstream: " + upstream
                            });
                        } else if (is_valid === false) {
                            await invoke("write_log", {
                                message: "Encounter " + key + " is not valid."
                            });
                        } else if (is_success === false) {
                            await invoke("write_log", {
                                message: "Failed to upload encounter " + key
                            });
                        }

                        await invoke("sync", {
                            encounter: Number(key),
                            upstream: upstream,
                            failed: !is_success,
                            isValid: is_valid
                        });

                        message = "Processing logs... (" + log_counter + "/" + ids.length + ")";
                    }
                }

                await delay(DELAY_TIME);
            }
        }

        // If there are any remaining encounters that did not fill a full batch
        if (encounters.length > 0) {
            const encountersInfo = await uploadLog(encounters, ids_batch, $settings.sync);

            for (const key in encountersInfo) {
                log_counter++;

                const upstream = encountersInfo[key].id;
                const is_success = encountersInfo[key].success;
                const is_valid = encountersInfo[key].is_valid;
                if (is_success === null || is_valid === null) {
                    break;
                }
                if (is_success === true && is_valid === true) {
                    synced++;

                    await invoke("write_log", {
                        message: "Uploaded encounter " + key + " upstream: " + upstream
                    });
                } else if (is_valid === false) {
                    await invoke("write_log", {
                        message: "Encounter " + key + " is not valid."
                    });
                } else if (is_success === false) {
                    await invoke("write_log", {
                        message: "Failed to upload encounter " + key
                    });
                }

                await invoke("sync", {
                    encounter: Number(key),
                    upstream: upstream,
                    failed: !is_success,
                    isValid: is_valid
                });

                message = "Processing logs... (" + log_counter + "/" + ids.length + ")";
            }
        }

        syncing = false;

        if (synced > 0) {
            message = "Uploaded " + synced + " logs.";
        } else {
            message = "No new logs were uploaded.";
        }
    }
</script>

<div class="flex flex-col space-y-4 divide-y-[1px]">
    <div class="mt-4 flex flex-col space-y-2 px-2">
        <SettingItem name="Sync (LOA Moon)" description="Enable log uploads" bind:setting={$settings.sync.enabled} />
        <p>Access Token</p>
        <input
            type="password"
            bind:value={$settings.sync.accessToken}
            class="focus:border-accent-500 block w-80 rounded-lg border border-gray-600 bg-zinc-700 text-xs text-zinc-300 placeholder-gray-400 focus:ring-0"
            placeholder="Insert your Access Token here" />
    </div>
    <div class="mt-4 flex flex-col space-y-2 px-2 pt-4">
        <SettingItem name="Auto Upload" description="Upload logs on clear" bind:setting={$settings.sync.auto} />
        <p>Excluded Characters</p>
        <div>
            <textarea
                bind:value={excludedCharacters}
                on:input={saveExcludedCharacters}
                class="focus:ring-accent-500 focus:border-accent-50 rounded-lg border border-gray-600 bg-gray-700 text-sm text-white" />
        </div>
        <p>Minimum Gear Score</p>
        <div>
            <select
                bind:value={$settings.sync.minGearScore}
                class="focus:ring-accent-500 focus:border-accent-500 yx-2 block w-28 rounded-lg border border-gray-600 bg-gray-700 py-1 text-sm text-white placeholder-gray-400">
                {#each ["1490", "1520", "1540", "1560", "1580", "1600", "1620", "1640"] as gearScore}
                    <option value={gearScore} selected>{gearScore}</option>
                {/each}
            </select>
        </div>
    </div>
    <div class="mt-4 flex flex-col space-y-2 px-2 pt-4">
        <div class="flex items-center space-x-2">
            <div>Sync All Past Logs:</div>
            {#if !syncing}
                <button class="rounded-md bg-zinc-600 p-1 hover:bg-zinc-700" on:click={syncPastLogs}>Sync</button>
            {:else}
                <button class="rounded-md bg-zinc-600 p-1 hover:bg-zinc-700" disabled>Syncing...</button>
            {/if}
        </div>
        {#if message}
            <p>{message}</p>
        {/if}
    </div>
</div>
