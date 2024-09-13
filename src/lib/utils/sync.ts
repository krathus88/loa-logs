import { invoke } from "@tauri-apps/api";
import pako from "pako";

export const bosses = [
    "Dark Mountain Predator",
    "Destroyer Lucas",
    "Leader Lugaru",
    "Demon Beast Commander Valtan",
    "Ravaged Tyrant of Beasts",
    "Incubus Morphe",
    "Nightmarish Morphe",
    "Covetous Devourer Vykas",
    "Covetous Legion Commander Vykas",
    "Saydon",
    "Kakul",
    "Kakul-Saydon",
    "Encore-Desiring Kakul-Saydon",
    "Gehenna Helkasirs",
    "Prokel",
    "Prokel's Spiritual Echo",
    "Ashtarot",
    "Primordial Nightmare",
    "Brelshaza, Monarch of Nightmares",
    "Phantom Legion Commander Brelshaza",
    "Griefbringer Maurug",
    "Evolved Maurug",
    "Lord of Degradation Akkan",
    "Plague Legion Commander Akkan",
    "Lord of Kartheon Akkan",
    "Tienis",
    "Celestial Sentinel",
    "Prunya",
    "Lauriel",
    "Kaltaya, the Blooming Chaos",
    "Rakathus, the Lurking Arrogance",
    "Firehorn, Trampler of Earth",
    "Lazaram, the Trailblazer",
    "Gargadeth",
    "Sonavel",
    "Hanumatan",
    "Kungelanium",
    "Deskaluda",
    "Behemoth, the Storm Commander",
    "Behemoth, Cruel Storm Slayer"
];

export async function uploadLog(encounter: any[], ids_batch: number[], settings: any) {
    const compressedData = pako.gzip(JSON.stringify(encounter));

    const resp = await fetch("https://loa-moon.onrender.com/api/log/", {
        method: "POST",
        headers: {
            Authorization: `Bearer ${settings.accessToken}`,
            "Content-Type": "application/octet-stream",
            "Content-Encoding": "gzip"
        },
        body: compressedData
    });

    if (!resp.ok && (resp.status == 500 || resp.status == 401)) {
        let error = "";
        if (resp.status == 500) {
            error = "server bonk";
        } else if (resp.status == 401) {
            error = "invalid access token";
        }

        await invoke("write_log", {
            message: `Couldn't upload encounters: ${ids_batch.join(", ")} due to end server issues. - error: ${error}`
        });
        return null;
    }

    const body = await resp.json();

    if (body.error) {
        await invoke("write_log", {
            message: `Couldn't upload encounters: ${ids_batch.join(", ")} due to end server issues. - error:  + ${body.error.toLowerCase()}`
        });
        for (const id of ids_batch) {
            await invoke("sync", { encounter: Number(id), upstream: 0, failed: true, isValid: true });
        }
        return null;
    }

    return body.encounters_info;
}
