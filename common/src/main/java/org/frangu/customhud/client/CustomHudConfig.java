package org.frangu.customhud.client;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;

import java.io.Reader;
import java.io.Writer;
import java.nio.charset.StandardCharsets;
import java.nio.file.AtomicMoveNotSupportedException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;

public final class CustomHudConfig {
    public boolean use12HourFormat = false;
    public boolean positionBottom = false;
    public boolean showDayCounter = true;
    public boolean showClock = true;
    public boolean showBossbar = true;

    private static final Gson GSON = new GsonBuilder().setPrettyPrinting().create();
    private static final String FILE_NAME = "customhud.json";
    private static final String LEGACY_FILE_NAME = "beautifulday_config.json";

    private static Path file;
    public static CustomHudConfig INSTANCE = new CustomHudConfig();

    private CustomHudConfig() {
    }

    public static void load(Path configDirectory) {
        file = configDirectory.resolve(FILE_NAME);
        Path legacyFile = configDirectory.resolve(LEGACY_FILE_NAME);
        Path sourceFile = Files.isRegularFile(file) ? file : Files.isRegularFile(legacyFile) ? legacyFile : null;

        if (sourceFile == null) {
            save();
            return;
        }

        try (Reader reader = Files.newBufferedReader(sourceFile, StandardCharsets.UTF_8)) {
            CustomHudConfig loaded = GSON.fromJson(reader, CustomHudConfig.class);
            INSTANCE = loaded != null ? loaded : new CustomHudConfig();
            if (sourceFile.equals(legacyFile)) {
                save();
            }
        } catch (Exception exception) {
            INSTANCE = new CustomHudConfig();
            System.err.println("[CustomHUD] Unable to load configuration: " + exception.getMessage());
        }
    }

    public static void save() {
        if (file == null) {
            return;
        }

        Path temporaryFile = file.resolveSibling(FILE_NAME + ".tmp");
        try {
            Files.createDirectories(file.getParent());
            try (Writer writer = Files.newBufferedWriter(temporaryFile, StandardCharsets.UTF_8)) {
                GSON.toJson(INSTANCE, writer);
            }

            try {
                Files.move(temporaryFile, file, StandardCopyOption.ATOMIC_MOVE, StandardCopyOption.REPLACE_EXISTING);
            } catch (AtomicMoveNotSupportedException ignored) {
                Files.move(temporaryFile, file, StandardCopyOption.REPLACE_EXISTING);
            }
        } catch (Exception exception) {
            System.err.println("[CustomHUD] Unable to save configuration: " + exception.getMessage());
        }
    }
}
