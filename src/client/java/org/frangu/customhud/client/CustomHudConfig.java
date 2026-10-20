package org.frangu.customhud.client;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import net.fabricmc.loader.api.FabricLoader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.Reader;
import java.io.Writer;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;

public class CustomHudConfig {
    private static final Logger LOGGER = LoggerFactory.getLogger("customhud");
    private static final Gson GSON = new GsonBuilder().setPrettyPrinting().create();
    private static final Path FILE = FabricLoader.getInstance().getConfigDir().resolve("beautifulday_config.json");

    public static CustomHudConfig INSTANCE = new CustomHudConfig();

    // Le nostre impostazioni (i campi mancanti nel file mantengono il valore di default)
    public boolean use12HourFormat = false;
    public boolean positionBottom = false; // Falso = In alto, Vero = In basso
    public boolean showDayCounter = true;
    public boolean showClock = true;
    public boolean showBossbar = true;

    public boolean isHudVisible() {
        return showDayCounter || showClock || showBossbar;
    }

    public static void load() {
        if (Files.exists(FILE)) {
            try (Reader reader = Files.newBufferedReader(FILE, StandardCharsets.UTF_8)) {
                CustomHudConfig loaded = GSON.fromJson(reader, CustomHudConfig.class);
                if (loaded != null) INSTANCE = loaded;
            } catch (Exception e) {
                LOGGER.error("Impossibile caricare la config, uso i valori di default", e);
            }
        }
        // Riscrive il file così eventuali nuove opzioni compaiono subito
        save();
    }

    public static void save() {
        try (Writer writer = Files.newBufferedWriter(FILE, StandardCharsets.UTF_8)) {
            GSON.toJson(INSTANCE, writer);
        } catch (Exception e) {
            LOGGER.error("Impossibile salvare la config", e);
        }
    }
}
