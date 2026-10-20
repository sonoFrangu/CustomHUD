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
    /** Di quanto vengono spostate in basso le bossbar vanilla quando l'HUD è in alto. */
    public static final int BOSS_BAR_SHIFT = 35;

    /**
     * Altezza dal fondo dello schermo occupata dalle barre di stato (vita, armatura, ossigeno),
     * fornita dal loader quando ne ha un'API (Fabric). 0 = usa solo i valori fissi.
     */
    public static IntSupplier statusBarHeight = () -> 0;

    // Colori della barra (RGB) nei quattro momenti chiave: mezzanotte, alba, mezzogiorno, tramonto
    private static final int PURPLE = 0x6633CC, YELLOW = 0xFFE633, BLUE = 0x3399FF, PINK = 0xFF4D80;

    // Creati al primo frame: i componenti degli item non sono pronti durante l'inizializzazione della mod
    private ItemStack clockIcon, creativeIcon, adventureIcon, spectatorIcon, goldenAppleIcon, totemIcon;

    // Cache testo
    private long cachedDays = Long.MIN_VALUE;
    private Language cachedLanguage;
    private Component dayText = Component.empty();
    private int dayWidth;
    private long cachedMinuteKey = Long.MIN_VALUE;
    private boolean cached12h;
    private String timeString = "";
    private int timeWidth;

    // Stato per il suono delle celebrazioni
    private ClientLevel lastLevel;
    private long lastSeenDays = -1;

    public void extract(GuiGraphicsExtractor g, DeltaTracker deltaTracker) {
        Minecraft mc = Minecraft.getInstance();
        LocalPlayer player = mc.player;
        ClientLevel level = mc.level;
        if (player == null || level == null || mc.gameMode == null) return;
        if (mc.gui.hud.isHidden() || mc.getDebugOverlay().showDebugScreen()) return;

        CustomHudConfig config = CustomHudConfig.INSTANCE;
        if (!config.isHudVisible()) return;

        if (clockIcon == null) createIcons();

        Font font = mc.font;
        long totalTime = level.getOverworldClockTime();
        long timeOfDay = Math.floorMod(totalTime, 24000L);
        long days = Math.floorDiv(totalTime, 24000L);

        updateDayText(font, days);
        updateTimeText(font, timeOfDay, config.use12HourFormat);

        boolean milestone = isMilestone(days);
        handleCelebrationSound(player, level, days, milestone);

        int screenWidth = g.guiWidth();
        int barX = (screenWidth - BAR_WIDTH) / 2;
        int barY = config.positionBottom ? g.guiHeight() - bottomOffset(player) : TOP_BAR_Y;
        int textY = barY - 12;

        int totalTextWidth = 0;
        if (config.showDayCounter) totalTextWidth += 13 + dayWidth;
        if (config.showDayCounter && config.showClock) totalTextWidth += 15;
        if (config.showClock) totalTextWidth += 13 + timeWidth;

        int x = (screenWidth - totalTextWidth) / 2;

        if (config.showDayCounter) {
            int textColor = 0xFFFFFFFF;
            if (milestone) {
                float hue = (System.currentTimeMillis() % 4000L) / 4000f;
                textColor = Mth.hsvToRgb(hue, 0.7f, 1.0f) | 0xFF000000;
            }
            drawModeIcon(g, mc, level, x, textY, days, milestone);
            x += 13;
            g.text(font, dayText, x, textY, textColor, true);
            x += dayWidth + 15;
        }

        if (config.showClock) {
            drawScaledItem(g, clockIcon, x, textY - 1);
            x += 13;
            g.text(font, timeString, x, textY, 0xFFFFFFFF, true);
        }

        if (config.showBossbar) {
            g.blitSprite(RenderPipelines.GUI_TEXTURED, BOSS_BAR_BACKGROUND, barX, barY, BAR_WIDTH, BAR_HEIGHT);
            long sinceMidnight = (timeOfDay + 6000L) % 24000L;
            int progressWidth = (int) (BAR_WIDTH * sinceMidnight / 24000L);
            if (progressWidth > 0) {
                g.blitSprite(RenderPipelines.GUI_TEXTURED, BAR_WHITE, BAR_WIDTH, BAR_HEIGHT, 0, 0,
                        barX, barY, progressWidth, BAR_HEIGHT, 0xFF000000 | smoothColor(sinceMidnight));
            }
        }
    }

    private void createIcons() {
        clockIcon = new ItemStack(Items.CLOCK);
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

    private static boolean isMilestone(long days) {
        return days > 0 && (days == 50 || days == 365 || days % 100 == 0);
    }

    /** Suona solo quando il giorno avanza davvero, non quando si entra in un mondo già al traguardo. */
    private void handleCelebrationSound(LocalPlayer player, ClientLevel level, long days, boolean milestone) {
        if (level != lastLevel) {
            lastLevel = level;
            lastSeenDays = days;
            return;
        }
        if (days > lastSeenDays && milestone) {
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

    private void drawModeIcon(GuiGraphicsExtractor g, Minecraft mc, ClientLevel level, int x, int y, long days, boolean milestone) {
        GameType mode = mc.gameMode.getPlayerMode();
        if (mode == GameType.SURVIVAL) {
            if (milestone) {
                drawScaledItem(g, days >= 500 ? totemIcon : goldenAppleIcon, x, y - 2);
            } else {
                // Ombra del cuore (nero semitrasparente) + cuore
                Identifier heart = level.getLevelData().isHardcore() ? HARDCORE_HEART_FULL : HEART_FULL;
                g.blitSprite(RenderPipelines.GUI_TEXTURED, heart, x + 1, y, 9, 9, 0x80000000);
                g.blitSprite(RenderPipelines.GUI_TEXTURED, heart, x, y - 1, 9, 9);
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

    private static int smoothColor(long ticks) {
        if (ticks < 6000) return lerpRgb(PURPLE, YELLOW, ticks / 6000f);
        if (ticks < 12000) return lerpRgb(YELLOW, BLUE, (ticks - 6000) / 6000f);
        if (ticks < 18000) return lerpRgb(BLUE, PINK, (ticks - 12000) / 6000f);
        return lerpRgb(PINK, PURPLE, (ticks - 18000) / 6000f);
    }

    private static int lerpRgb(int a, int b, float t) {
        int r = (int) Mth.lerp(t, (a >> 16) & 0xFF, (b >> 16) & 0xFF);
        int gr = (int) Mth.lerp(t, (a >> 8) & 0xFF, (b >> 8) & 0xFF);
        int bl = (int) Mth.lerp(t, a & 0xFF, b & 0xFF);
        return (r << 16) | (gr << 8) | bl;
    }
}
