import { invoke } from "@tauri-apps/api";
import pako from "pako";

export const bosses = [
    // Legion Raids
    // Valtan
    "Dark Mountain Predator",
    "Ravaged Tyrant of Beasts",
    // Vykas
    "Nightmarish Morphe",
    "Covetous Devourer Vykas",
    "Covetous Legion Commander Vykas",
    // Kakul-Saydon
    "Saydon",
    "Kakul",
    "Encore-Desiring Kakul-Saydon",
    // Brelshaza
    "Gehenna Helkasirs",
    "Ashtarot",
    "Primordial Nightmare",
    "Phantom Legion Commander Brelshaza",
    // Akkan
    "Evolved Maurug",
    "Lord of Degradation Akkan",
    "Plague Legion Commander Akkan",
    "Lord of Kartheon Akkan",
    // Thaemine
    "Killineza the Dark Worshipper",
    "Valinak, Herald of the End",
    "Thaemine the Lightqueller",
    "Thaemine, Conqueror of Stars",
    // Echidna,
    "Red Doom Narkiel",
    "Covetous Master Echidna",
    // Behemoth
    "Behemoth, the Storm Commander",
    "Behemoth, Cruel Storm Slayer",
    // Abyssal Dungeons
    // Kayangel
    "Tienis",
    "Prunya",
    "Lauriel",
    // Ivory Tower
    "Kaltaya, the Blooming Chaos",
    "Rakathus, the Lurking Arrogance",
    "Lazaram, the Trailblazer",
    // Guardian Raids - Trial
    "Achates",
    "Caliligos",
    "Hanumatan"
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
