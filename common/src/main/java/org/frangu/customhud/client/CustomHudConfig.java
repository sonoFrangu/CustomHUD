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
import java.util.List;

public final class CustomHudConfig {
    public enum BarTheme { CLASSIC, PASTEL, COLORBLIND, MONO }

    /** Cosa mostrare di ora e barra fuori dall'Overworld, dove il ciclo giorno/notte non c'è. */
    public enum OtherDimensions { SHOW, UNKNOWN, HIDE }

    // Elementi
    public boolean showDayCounter = true;
    public boolean showClock = true;
    public boolean showBossbar = true;
    public boolean showNightTicks = false;
    public boolean showSleepIcon = false;
    public boolean showMoonPhase = false;
    public boolean showNightCountdown = false;
    public boolean showWeather = false;
    public boolean phantomWarning = true;

    // Aspetto
    public int scale = 100;
    public int opacity = 100;
    public boolean showBackground = false;
    public BarTheme barTheme = BarTheme.CLASSIC;

    // Comportamento
    public boolean use12HourFormat = false;
    public boolean positionBottom = false;
    public int offsetX = 0;
    public int offsetY = 0;
    public OtherDimensions otherDimensions = OtherDimensions.SHOW;
    public boolean milestones = true;
    public boolean milestoneSound = true;
    /** Ogni quanti giorni c'è un traguardo; 0 = i classici (50, 365 e ogni 100). */
    public int milestoneEvery = 0;

    // Modalità semplice: solo "Giorno: N", in un punto qualsiasi dello schermo
    // (percentuale dello spazio libero, da sinistra e dall'alto).
    public boolean simpleMode = false;
    public int simpleX = 50;
    public int simpleY = 77;

    public static final int MIN_SCALE = 75, MAX_SCALE = 150;
    public static final int MIN_OPACITY = 20, MAX_OPACITY = 100;
    public static final int MAX_OFFSET = 100;
    /** Le scelte di "Traguardo ogni", nell'ordine del pulsante. */
    public static final List<Integer> MILESTONE_EVERY = List.of(0, 10, 25, 50, 100, 250, 365, 500, 1000);

    private static final Gson GSON = new GsonBuilder().setPrettyPrinting().create();
    private static final String FILE_NAME = "customhud.json";
    private static final String LEGACY_FILE_NAME = "beautifulday_config.json";

    private static Path file;
    public static CustomHudConfig INSTANCE = new CustomHudConfig();

    private CustomHudConfig() {
    }

    public boolean isHudVisible() {
        return simpleMode || showDayCounter || showClock || showBossbar;
    }

    /** Il giorno è un traguardo: i classici (50, 365 e ogni 100) o uno ogni {@code milestoneEvery} giorni. */
    public boolean isMilestone(long days) {
        if (days <= 0) return false;
        if (milestoneEvery > 0) return days % milestoneEvery == 0;
        return days == 50 || days == 365 || days % 100 == 0;
    }

    public static void reset() {
        INSTANCE = new CustomHudConfig();
        save();
    }

    /** Riporta nei limiti i valori modificati a mano nel file. */
    private void sanitize() {
        scale = Math.clamp(scale, MIN_SCALE, MAX_SCALE);
        opacity = Math.clamp(opacity, MIN_OPACITY, MAX_OPACITY);
        offsetX = Math.clamp(offsetX, -MAX_OFFSET, MAX_OFFSET);
        offsetY = Math.clamp(offsetY, -MAX_OFFSET, MAX_OFFSET);
        simpleX = Math.clamp(simpleX, 0, 100);
        simpleY = Math.clamp(simpleY, 0, 100);
        if (!MILESTONE_EVERY.contains(milestoneEvery)) milestoneEvery = 0;
        if (barTheme == null) barTheme = BarTheme.CLASSIC;
        if (otherDimensions == null) otherDimensions = OtherDimensions.SHOW;
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
            INSTANCE.sanitize();
            // Riscrive il file così le nuove opzioni compaiono subito
            save();
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
