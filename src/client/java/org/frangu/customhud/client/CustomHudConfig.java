package org.frangu.customhud.client;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import net.fabricmc.loader.api.FabricLoader;

import java.io.File;
import java.io.FileReader;
import java.io.FileWriter;

public class CustomHudConfig {
    // Le nostre impostazioni
    public boolean use12HourFormat = false;
    public boolean positionBottom = false; // Falso = In alto, Vero = In basso
    public boolean showDayCounter = true;
    public boolean showClock = true;
    public boolean showBossbar = true;

    private static final Gson GSON = new GsonBuilder().setPrettyPrinting().create();
    private static final File FILE = new File(FabricLoader.getInstance().getConfigDir().toFile(), "beautifulday_config.json");
    public static CustomHudConfig INSTANCE = new CustomHudConfig();

    public static void load() {
        if (FILE.exists()) {
            try (FileReader reader = new FileReader(FILE)) {
                INSTANCE = GSON.fromJson(reader, CustomHudConfig.class);
            } catch (Exception e) {
                System.out.println("Errore nel caricamento della config!");
            }
        } else {
            save();
        }
    }

    public static void save() {
        try (FileWriter writer = new FileWriter(FILE)) {
            GSON.toJson(INSTANCE, writer);
        } catch (Exception e) {
            System.out.println("Errore nel salvataggio della config!");
        }
    }
}