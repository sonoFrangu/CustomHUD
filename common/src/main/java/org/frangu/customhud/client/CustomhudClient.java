package org.frangu.customhud.client;

import com.mojang.blaze3d.platform.InputConstants;
import net.minecraft.client.DeltaTracker;
import net.minecraft.client.KeyMapping;
import net.minecraft.client.Minecraft;
import net.minecraft.client.gui.GuiGraphicsExtractor;
import net.minecraft.client.player.LocalPlayer;
import net.minecraft.client.gui.screens.Screen;
import net.minecraft.network.chat.Component;

import java.nio.file.Path;

public final class CustomhudClient {
    public static KeyMapping configKeyMapping;
    private static long lastCelebratedDay = -1;

    private CustomhudClient() {
    }

    public static void init(Path configDirectory) {
        CustomHudConfig.load(configDirectory);
        configKeyMapping = new KeyMapping(
                "key.customhud.open_config",
                InputConstants.Type.KEYSYM,
                InputConstants.KEY_H,
                KeyMapping.Category.MISC
        );
    }

    public static void onEndClientTick(Minecraft client) {
        if (configKeyMapping == null) {
            return;
        }

        while (configKeyMapping.consumeClick()) {
            Screen parent = client.screen;
            client.setScreen(new ConfigScreen(parent));
        }
    }

    public static void extractHud(GuiGraphicsExtractor graphics, DeltaTracker deltaTracker) {
        Minecraft client = Minecraft.getInstance();
        LocalPlayer player = client.player;

        if (player == null || client.level == null || client.options.hideGui) {
            return;
        }

        CustomHudConfig config = CustomHudConfig.INSTANCE;
        if (!config.showDayCounter && !config.showClock && !config.showBossbar) {
            return;
        }

        long gameTime = client.level.getGameTime();
        long timeOfDay = Math.floorMod(client.level.getOverworldClockTime(), 24000L);
        long days = Math.max(0L, gameTime / 24000L);
        int hours24 = (int) ((timeOfDay / 1000L + 6L) % 24L);
        int minutes = (int) ((timeOfDay % 1000L) * 60L / 1000L);

        String timeText = config.use12HourFormat
                ? String.format("%d:%02d %s", hours24 % 12 == 0 ? 12 : hours24 % 12, minutes, hours24 < 12 ? "AM" : "PM")
                : String.format("%02d:%02d", hours24, minutes);

        Component dayText = Component.translatable("hud.customhud.day", days);
        int width = graphics.guiWidth();
        int height = graphics.guiHeight();
        int barWidth = 182;
        int barHeight = 5;
        int barX = (width - barWidth) / 2;
        int barY = config.positionBottom ? height - 42 : 25;
        int textY = barY - 12;

        int totalTextWidth = 0;
        if (config.showDayCounter) {
            totalTextWidth += client.font.width(dayText) + 4;
        }
        if (config.showDayCounter && config.showClock) {
            totalTextWidth += 10;
        }
        if (config.showClock) {
            totalTextWidth += client.font.width(timeText) + 4;
        }

        int currentX = (width - totalTextWidth) / 2;
        if (config.showDayCounter) {
            boolean milestone = days > 0 && (days == 50 || days == 100 || days == 365 || days % 100 == 0);
            int dayColor = milestone ? milestoneColor() : 0xFFFFFFFF;
            if (milestone && lastCelebratedDay != days) {
                lastCelebratedDay = days;
                player.playSound(net.minecraft.sounds.SoundEvents.PLAYER_LEVELUP, 0.6F, 1.0F);
            } else if (!milestone) {
                lastCelebratedDay = -1;
            }

            graphics.text(client.font, dayText, currentX, textY, dayColor, true);
            currentX += client.font.width(dayText) + 14;
        }

        if (config.showClock) {
            graphics.text(client.font, timeText, currentX, textY, 0xFFFFFFFF, true);
        }

        if (config.showBossbar) {
            graphics.fill(barX, barY, barX + barWidth, barY + barHeight, 0x66000000);
            int progressWidth = (int) (barWidth * (timeOfDay / 24000.0F));
            if (progressWidth > 0) {
                graphics.fill(barX, barY, barX + progressWidth, barY + barHeight, dayCycleColor(timeOfDay));
            }
        }
    }

    private static int milestoneColor() {
        float phase = (System.currentTimeMillis() % 4000L) / 4000.0F;
        int red = (int) (127.0F + 127.0F * (float) Math.sin(phase * Math.PI * 2.0F));
        int green = (int) (127.0F + 127.0F * (float) Math.sin((phase + 0.333F) * Math.PI * 2.0F));
        int blue = (int) (127.0F + 127.0F * (float) Math.sin((phase + 0.666F) * Math.PI * 2.0F));
        return 0xFF000000 | red << 16 | green << 8 | blue;
    }

    private static int dayCycleColor(long timeOfDay) {
        float phase = timeOfDay / 24000.0F;
        int red = (int) (255.0F * (0.5F + 0.5F * (float) Math.sin((phase + 0.0F) * Math.PI * 2.0F)));
        int green = (int) (255.0F * (0.5F + 0.5F * (float) Math.sin((phase + 0.33F) * Math.PI * 2.0F)));
        int blue = (int) (255.0F * (0.5F + 0.5F * (float) Math.sin((phase + 0.66F) * Math.PI * 2.0F)));
        return 0xFF000000 | red << 16 | green << 8 | blue;
    }
}
