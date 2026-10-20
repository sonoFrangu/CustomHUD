package org.frangu.customhud.client;

import net.minecraft.client.DeltaTracker;
import net.minecraft.client.Minecraft;
import net.minecraft.client.gui.Font;
import net.minecraft.client.gui.GuiGraphicsExtractor;
import net.minecraft.client.multiplayer.ClientLevel;
import net.minecraft.client.player.LocalPlayer;
import net.minecraft.client.renderer.RenderPipelines;
import net.minecraft.locale.Language;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.Identifier;
import net.minecraft.sounds.SoundEvents;
import net.minecraft.util.Mth;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.Level;
import org.frangu.customhud.client.CustomHudConfig.BarTheme;
import org.frangu.customhud.client.CustomHudConfig.OtherDimensions;
import org.joml.Matrix3x2fStack;

import java.util.function.IntSupplier;

/**
 * Disegna contatore dei giorni, orologio e barra del ciclo giorno/notte.
 * Le stringhe vengono ricalcolate solo quando cambia il giorno o il minuto,
 * non a ogni frame.
 */
public final class DayHud {

    private static final Identifier HEART_FULL = Identifier.withDefaultNamespace("hud/heart/full");
    private static final Identifier HARDCORE_HEART_FULL = Identifier.withDefaultNamespace("hud/heart/hardcore_full");
    private static final Identifier BOSS_BAR_BACKGROUND = Identifier.withDefaultNamespace("boss_bar/blue_background");
    private static final Identifier BAR_WHITE = Identifier.withDefaultNamespace("boss_bar/white_progress");

    private static final int BAR_WIDTH = 182;
    private static final int BAR_HEIGHT = 5;
    private static final int TOP_BAR_Y = 25;
    /** Di quanto vengono spostate in basso le bossbar vanilla quando l'HUD è in alto, a scala 100%. */
    private static final int BOSS_BAR_SHIFT = 35;

    /**
     * Altezza dal fondo dello schermo occupata dalle barre di stato (vita, armatura, ossigeno),
     * fornita dal loader quando ne ha un'API (Fabric). 0 = usa solo i valori fissi.
     */
    public static IntSupplier statusBarHeight = () -> 0;

    /**
     * Colori della barra (RGB) per tick da mezzanotte. Le tappe intermedie (aurora, sole del mattino)
     * evitano che l'interpolazione RGB passi per toni grigi o marroni.
     */
    private static final long[] COLOR_TICKS = {0, 4500, 6000, 7500, 12000, 18000, 24000};
    private static final int[] CLASSIC = {0x6633CC, 0xFF7A4D, 0xFFE633, 0xFFF5C0, 0x3399FF, 0xFF4D80, 0x6633CC};
    private static final int[] PASTEL = {0x9F8FD9, 0xFFB59E, 0xFFF0A0, 0xFFFBE0, 0xA8D8FF, 0xFFB3C8, 0x9F8FD9};
    // Palette Okabe-Ito: distinguibile anche con daltonismo rosso/verde
    private static final int[] COLORBLIND = {0x2B3A8C, 0xE69F00, 0xF0E442, 0xF5EEA0, 0x56B4E9, 0xD55E00, 0x2B3A8C};
    private static final int[] MONO = {0x3A3A3A, 0x8A8A8A, 0xD0D0D0, 0xEEEEEE, 0xFFFFFF, 0x9A9A9A, 0x3A3A3A};

    /** Finestra in cui si può dormire con tempo sereno (tick del giorno, come in vanilla). */
    private static final long SLEEP_START = 12542, SLEEP_END = 23460;

    private static final int MOON_SIZE = 8;

    // Creati al primo frame: i componenti degli item non sono pronti durante l'inizializzazione della mod
    private ItemStack clockIcon, bedIcon, creativeIcon, adventureIcon, spectatorIcon, goldenAppleIcon, totemIcon;

    // Cache testo
    private long cachedDays = Long.MIN_VALUE;
    private Language cachedLanguage;
    private Component dayText = Component.empty();
    private int dayWidth;
    private long cachedMinuteKey = Long.MIN_VALUE;
    private boolean cached12h;
    private String timeString = "";
    private int timeWidth;
    private long cachedCountdownKey = Long.MIN_VALUE;
    private Language cachedCountdownLanguage;
    private String countdownString = "";
    private int countdownWidth;

    // Stato per il suono delle celebrazioni
    private ClientLevel lastLevel;
    private long lastSeenDays = -1;

    public void extract(GuiGraphicsExtractor g, DeltaTracker deltaTracker) {
        Minecraft mc = Minecraft.getInstance();
        LocalPlayer player = mc.player;
        ClientLevel level = mc.level;
        if (player == null || level == null || mc.gameMode == null) return;
        if (mc.gui.hud.isHidden() || mc.getDebugOverlay().showDebugScreen()) return;
        // Nel menu delle impostazioni l'HUD finirebbe sopra al titolo
        if (mc.gui.screen() instanceof ConfigScreen) return;

        CustomHudConfig config = CustomHudConfig.INSTANCE;

        // Fuori dall'Overworld l'ora non ha senso: si può nascondere o mostrare "??:??"
        boolean overworld = level.dimension() == Level.OVERWORLD;
        boolean hideTime = !overworld && config.otherDimensions == OtherDimensions.HIDE;
        boolean unknownTime = !overworld && config.otherDimensions == OtherDimensions.UNKNOWN;
        boolean timeExtras = !hideTime && !unknownTime;

        boolean showDays = config.showDayCounter;
        boolean showClock = config.showClock && !hideTime;
        boolean showBar = config.showBossbar && !hideTime;
        boolean showCountdown = config.showNightCountdown && timeExtras;
        boolean showMoon = config.showMoonPhase && timeExtras;
        if (!showDays && !showClock && !showBar) return;

        if (clockIcon == null) createIcons();

        Font font = mc.font;
        long totalTime = level.getOverworldClockTime();
        long timeOfDay = Math.floorMod(totalTime, 24000L);
        long days = Math.floorDiv(totalTime, 24000L);

        updateDayText(font, days);
        if (unknownTime) {
            timeString = "??:??";
            timeWidth = font.width(timeString);
            cachedMinuteKey = Long.MIN_VALUE;
        } else {
            updateTimeText(font, timeOfDay, config.use12HourFormat);
        }
        if (showCountdown) updateCountdownText(font, timeOfDay);

        boolean milestone = config.milestones && isMilestone(days);
        handleCelebrationSound(player, level, days, milestone && config.milestoneSound);

        int alpha = Math.round(config.opacity * 2.55f);
        int white = alpha << 24 | 0xFFFFFF;
        int screenWidth = g.guiWidth();
        int centerX = screenWidth / 2;
        int barX = (screenWidth - BAR_WIDTH) / 2;
        int barY = config.positionBottom ? g.guiHeight() - bottomOffset(player) : TOP_BAR_Y;
        int textY = barY - 12;

        int totalTextWidth = 0;
        if (showDays) totalTextWidth += 13 + dayWidth;
        if (showClock) totalTextWidth += (totalTextWidth > 0 ? 15 : 0) + 13 + timeWidth;
        if (showCountdown) totalTextWidth += (totalTextWidth > 0 ? 6 : 0) + countdownWidth;
        if (showMoon) totalTextWidth += (totalTextWidth > 0 ? 6 : 0) + MOON_SIZE;
        boolean hasText = totalTextWidth > 0;

        // Scala attorno al bordo dello schermo più vicino, poi sposta di quanto scelto dall'utente
        float scale = config.scale / 100f;
        float anchorY = config.positionBottom ? barY + BAR_HEIGHT : 0;
        Matrix3x2fStack pose = g.pose();
        pose.pushMatrix();
        pose.translate(centerX + config.offsetX, anchorY + config.offsetY);
        pose.scale(scale, scale);
        pose.translate(-centerX, -anchorY);

        if (config.showBackground) {
            int width = Math.max(totalTextWidth, showBar ? BAR_WIDTH : 0);
            int top = hasText ? textY - 4 : barY - 3;
            int bottom = showBar ? barY + BAR_HEIGHT + 3 : textY + 11;
            g.fill(centerX - width / 2 - 4, top, centerX + (width + 1) / 2 + 4, bottom, (alpha * 0x90 / 0xFF) << 24);
        }

        int x = (screenWidth - totalTextWidth) / 2;

        if (showDays) {
            int textColor = white;
            if (milestone) {
                float hue = (System.currentTimeMillis() % 4000L) / 4000f;
                textColor = Mth.hsvToRgb(hue, 0.7f, 1.0f) & 0xFFFFFF | alpha << 24;
            }
            drawModeIcon(g, mc, level, x, textY, days, milestone, alpha);
            x += 13;
            g.text(font, dayText, x, textY, textColor, true);
            x += dayWidth;
        }

        if (showClock) {
            if (showDays) x += 15;
            boolean sleepIcon = config.showSleepIcon && timeExtras && canSleep(level, timeOfDay);
            drawScaledItem(g, sleepIcon ? bedIcon : clockIcon, x, textY - 1);
            x += 13;
            g.text(font, timeString, x, textY, white, true);
            x += timeWidth;
        }

        if (showCountdown) {
            if (showDays || showClock) x += 6;
            g.text(font, countdownString, x, textY, alpha << 24 | 0xBBBBBB, true);
            x += countdownWidth;
        }

        if (showMoon) {
            if (showDays || showClock || showCountdown) x += 6;
            drawMoon(g, x, textY, (int) Math.floorMod(days, 8L), alpha);
        }

        if (showBar) {
            g.blitSprite(RenderPipelines.GUI_TEXTURED, BOSS_BAR_BACKGROUND, barX, barY, BAR_WIDTH, BAR_HEIGHT, white);
            if (!unknownTime) {
                long sinceMidnight = (timeOfDay + 6000L) % 24000L;
                int progressWidth = (int) (BAR_WIDTH * sinceMidnight / 24000L);
                if (progressWidth > 0) {
                    g.blitSprite(RenderPipelines.GUI_TEXTURED, BAR_WHITE, BAR_WIDTH, BAR_HEIGHT, 0, 0,
                            barX, barY, progressWidth, BAR_HEIGHT, alpha << 24 | barColor(config.barTheme, sinceMidnight));
                }
                if (config.showNightTicks) {
                    drawBarTick(g, barX, barY, SLEEP_START, alpha);
                    drawBarTick(g, barX, barY, SLEEP_END, alpha);
                }
            }
        }

        pose.popMatrix();
    }

    /** Di quanto spostare le bossbar vanilla perché non finiscano sotto l'HUD in alto. */
    public static int bossBarShift() {
        CustomHudConfig config = CustomHudConfig.INSTANCE;
        return Math.max(0, Math.round(BOSS_BAR_SHIFT * config.scale / 100f) + config.offsetY);
    }

    private void createIcons() {
        clockIcon = new ItemStack(Items.CLOCK);
        bedIcon = new ItemStack(Items.BED.red());
        creativeIcon = new ItemStack(Items.GRASS_BLOCK);
        adventureIcon = new ItemStack(Items.IRON_SWORD);
        spectatorIcon = new ItemStack(Items.ENDER_EYE);
        goldenAppleIcon = new ItemStack(Items.GOLDEN_APPLE);
        totemIcon = new ItemStack(Items.TOTEM_OF_UNDYING);
    }

    private void updateDayText(Font font, long days) {
        Language language = Language.getInstance();
        if (days == cachedDays && language == cachedLanguage) return;
        cachedDays = days;
        cachedLanguage = language;
        dayText = Component.literal(Component.translatable("hud.customhud.day", days).getString());
        dayWidth = font.width(dayText);
    }

    private void updateTimeText(Font font, long timeOfDay, boolean use12h) {
        long hours24 = (timeOfDay / 1000L + 6) % 24;
        long minutes = (timeOfDay % 1000L) * 60 / 1000L;
        long minuteKey = hours24 * 60 + minutes;
        if (minuteKey == cachedMinuteKey && use12h == cached12h) return;
        cachedMinuteKey = minuteKey;
        cached12h = use12h;
        timeString = use12h
                ? String.format("%d:%02d %s", hours24 % 12 == 0 ? 12 : hours24 % 12, minutes, hours24 < 12 ? "AM" : "PM")
                : String.format("%02d:%02d", hours24, minutes);
        timeWidth = font.width(timeString);
    }

    /** "Notte tra 4:20" di giorno, "Giorno tra 1:05" di notte, in tempo reale (minuti:secondi). */
    private void updateCountdownText(Font font, long timeOfDay) {
        boolean night = timeOfDay >= SLEEP_START && timeOfDay < SLEEP_END;
        long ticksLeft = night ? SLEEP_END - timeOfDay : Math.floorMod(SLEEP_START - timeOfDay, 24000L);
        long seconds = ticksLeft / 20;
        long key = night ? -1 - seconds : seconds;
        Language language = Language.getInstance();
        if (key == cachedCountdownKey && language == cachedCountdownLanguage) return;
        cachedCountdownKey = key;
        cachedCountdownLanguage = language;
        String left = String.format("%d:%02d", seconds / 60, seconds % 60);
        countdownString = Component.translatable(night ? "hud.customhud.day_in" : "hud.customhud.night_in", left).getString();
        countdownWidth = font.width(countdownString);
    }

    private static boolean isMilestone(long days) {
        return days > 0 && (days == 50 || days == 365 || days % 100 == 0);
    }

    /** Suona solo quando il giorno avanza davvero, non quando si entra in un mondo già al traguardo. */
    private void handleCelebrationSound(LocalPlayer player, ClientLevel level, long days, boolean playSound) {
        if (level != lastLevel) {
            lastLevel = level;
            lastSeenDays = days;
            return;
        }
        if (days > lastSeenDays && playSound) {
            player.playSound(SoundEvents.PLAYER_LEVELUP, 0.6f, 1.0f);
        }
        lastSeenDays = days;
    }

    /**
     * Distanza dal fondo dello schermo. Usa gli stessi valori di prima, ma sale ancora
     * se le barre di stato sono più alte (molti cuori di assorbimento, barre di altre mod).
     */
    private static int bottomOffset(LocalPlayer player) {
        if (player.isSpectator()) return 15;
        if (player.isCreative()) return 45;
        int legacy = player.getArmorValue() > 0 || player.getAirSupply() < player.getMaxAirSupply() ? 70 : 55;
        int statusBars = statusBarHeight.getAsInt();
        return statusBars > 0 ? Math.max(legacy, statusBars + 16) : legacy;
    }

    private void drawModeIcon(GuiGraphicsExtractor g, Minecraft mc, ClientLevel level, int x, int y, long days,
                              boolean milestone, int alpha) {
        GameType mode = mc.gameMode.getPlayerMode();
        if (mode == GameType.SURVIVAL) {
            if (milestone) {
                drawScaledItem(g, days >= 500 ? totemIcon : goldenAppleIcon, x, y - 2);
            } else {
                // Ombra del cuore (nero semitrasparente) + cuore
                Identifier heart = level.getLevelData().isHardcore() ? HARDCORE_HEART_FULL : HEART_FULL;
                g.blitSprite(RenderPipelines.GUI_TEXTURED, heart, x + 1, y, 9, 9, (alpha / 2) << 24);
                g.blitSprite(RenderPipelines.GUI_TEXTURED, heart, x, y - 1, 9, 9, alpha << 24 | 0xFFFFFF);
            }
            return;
        }
        ItemStack icon = switch (mode) {
            case CREATIVE -> creativeIcon;
            case ADVENTURE -> adventureIcon;
            case SPECTATOR -> spectatorIcon;
            default -> ItemStack.EMPTY;
        };
        if (!icon.isEmpty()) drawScaledItem(g, icon, x, y - 1);
    }

    private static void drawScaledItem(GuiGraphicsExtractor g, ItemStack stack, int x, int y) {
        Matrix3x2fStack pose = g.pose();
        pose.pushMatrix();
        pose.translate(x, y);
        pose.scale(0.6f, 0.6f);
        g.item(stack, 0, 0);
        pose.popMatrix();
    }

    /** Nell'Overworld si dorme di notte o durante un temporale. */
    private static boolean canSleep(ClientLevel level, long timeOfDay) {
        if (level.dimension() != Level.OVERWORLD) return false;
        return level.isThundering() || (timeOfDay >= SLEEP_START && timeOfDay < SLEEP_END);
    }

    /** Trattino verticale sulla barra all'ora indicata (tick del giorno). */
    private static void drawBarTick(GuiGraphicsExtractor g, int barX, int barY, long timeOfDay, int alpha) {
        int x = barX + (int) (BAR_WIDTH * ((timeOfDay + 6000L) % 24000L) / 24000L);
        g.fill(x, barY - 1, x + 1, barY + BAR_HEIGHT + 1, (alpha * 0xB0 / 0xFF) << 24 | 0xFFFFFF);
    }

    /**
     * Luna 8x8 disegnata riga per riga: parte illuminata chiara, parte in ombra scura.
     * Fase 0 = luna piena, 4 = luna nuova (come in vanilla); 1-3 calante, 5-7 crescente.
     */
    private static void drawMoon(GuiGraphicsExtractor g, int x, int y, int phase, int alpha) {
        int lit = alpha << 24 | 0xF2EFD9;
        int dark = (alpha * 3 / 4) << 24 | 0x2A2A3A;
        double terminator = Math.cos(phase * Math.PI / 4);
        int center = x + MOON_SIZE / 2;
        for (int row = 0; row < MOON_SIZE; row++) {
            double dy = row - (MOON_SIZE - 1) / 2.0;
            int half = (int) Math.round(Math.sqrt(MOON_SIZE * MOON_SIZE / 4.0 - dy * dy));
            int split = Mth.clamp((int) Math.round(terminator * half), -half, half);
            int top = y + row;
            if (phase <= 4) {
                // Calante: la luce resta a sinistra
                fillRow(g, center - half, center + split, top, lit);
                fillRow(g, center + split, center + half, top, dark);
            } else {
                // Crescente: la luce cresce da destra
                fillRow(g, center - half, center - split, top, dark);
                fillRow(g, center - split, center + half, top, lit);
            }
        }
    }

    private static void fillRow(GuiGraphicsExtractor g, int from, int to, int y, int color) {
        if (from < to) g.fill(from, y, to, y + 1, color);
    }

    private static int barColor(BarTheme theme, long ticks) {
        int[] colors = switch (theme) {
            case PASTEL -> PASTEL;
            case COLORBLIND -> COLORBLIND;
            case MONO -> MONO;
            default -> CLASSIC;
        };
        int i = 1;
        while (i < COLOR_TICKS.length - 1 && ticks >= COLOR_TICKS[i]) i++;
        float t = (float) (ticks - COLOR_TICKS[i - 1]) / (COLOR_TICKS[i] - COLOR_TICKS[i - 1]);
        return lerpRgb(colors[i - 1], colors[i], t);
    }

    private static int lerpRgb(int a, int b, float t) {
        int r = (int) Mth.lerp(t, (a >> 16) & 0xFF, (b >> 16) & 0xFF);
        int gr = (int) Mth.lerp(t, (a >> 8) & 0xFF, (b >> 8) & 0xFF);
        int bl = (int) Mth.lerp(t, a & 0xFF, b & 0xFF);
        return (r << 16) | (gr << 8) | bl;
    }
}
