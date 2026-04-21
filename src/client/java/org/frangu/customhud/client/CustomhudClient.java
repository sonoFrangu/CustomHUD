package org.frangu.customhud.client;

import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents;
import net.fabricmc.fabric.api.client.keybinding.v1.KeyBindingHelper;
import net.fabricmc.fabric.api.client.rendering.v1.HudRenderCallback;
import net.minecraft.client.MinecraftClient;
import net.minecraft.client.network.ClientPlayerEntity;
import net.minecraft.client.option.KeyBinding;
import net.minecraft.client.util.InputUtil;
import net.minecraft.item.ItemStack;
import net.minecraft.item.Items;
import net.minecraft.text.Text;
import net.minecraft.util.Identifier;
import net.minecraft.util.math.MathHelper;
import net.minecraft.world.GameMode;
import net.minecraft.client.gl.RenderPipelines;
import net.minecraft.sound.SoundEvents;
import org.lwjgl.glfw.GLFW;

public class CustomhudClient implements ClientModInitializer {

    private static final Identifier HEART_FULL = Identifier.of("minecraft", "hud/heart/full");
    private static final Identifier HARDCORE_HEART_FULL = Identifier.of("minecraft", "hud/heart/hardcore_full");
    private static final Identifier BOSS_BAR_BACKGROUND = Identifier.of("minecraft", "boss_bar/blue_background");
    private static final Identifier BAR_WHITE = Identifier.of("minecraft", "boss_bar/white_progress");

    private GameMode lastGameMode = null;
    private ItemStack cachedIconItem = ItemStack.EMPTY;
    private long lastCelebratedDay = -1;

    @Override
    public void onInitializeClient() {
        CustomHudConfig.load();

        KeyBinding configKeyBinding = KeyBindingHelper.registerKeyBinding(new KeyBinding(
                "key.customhud.open_config",
                InputUtil.Type.KEYSYM,
                GLFW.GLFW_KEY_H,
                KeyBinding.Category.MISC
        ));

        ClientTickEvents.END_CLIENT_TICK.register(client -> {
            while (configKeyBinding.wasPressed()) {
                client.setScreen(new ConfigScreen(client.currentScreen));
            }
        });

        HudRenderCallback.EVENT.register((drawContext, tickCounter) -> {
            MinecraftClient client = MinecraftClient.getInstance();
            ClientPlayerEntity player = client.player;

            if (player == null || client.world == null) return;
            if (client.options.hudHidden || client.getDebugHud().shouldShowDebugHud()) return;

            CustomHudConfig config = CustomHudConfig.INSTANCE;
            if (!config.showDayCounter && !config.showClock && !config.showBossbar) return;

            GameMode currentGameMode = client.interactionManager.getCurrentGameMode();
            boolean isHardcore = client.world.getLevelProperties().isHardcore();
            if (currentGameMode != lastGameMode) {
                lastGameMode = currentGameMode;
                if (currentGameMode == GameMode.CREATIVE) cachedIconItem = new ItemStack(Items.GRASS_BLOCK);
                else if (currentGameMode == GameMode.ADVENTURE) cachedIconItem = new ItemStack(Items.IRON_SWORD);
                else if (currentGameMode == GameMode.SPECTATOR) cachedIconItem = new ItemStack(Items.ENDER_EYE);
                else cachedIconItem = ItemStack.EMPTY;
            }

            long totalTime = client.world.getTimeOfDay();
            long timeOfDay = totalTime % 24000L;
            long hours24 = (timeOfDay / 1000L + 6) % 24;
            long minutes = (timeOfDay % 1000L) * 60 / 1000L;
            long days = totalTime / 24000L;

            String timeText = config.use12HourFormat
                    ? String.format("%d:%02d %s", (hours24 % 12 == 0 ? 12 : hours24 % 12), minutes, (hours24 < 12 ? "AM" : "PM"))
                    : String.format("%02d:%02d", hours24, minutes);

            Text dayText = Text.translatable("hud.customhud.day", days);

            int screenWidth = client.getWindow().getScaledWidth();
            int screenHeight = client.getWindow().getScaledHeight();
            int barWidth = 182;
            int barX = (screenWidth - barWidth) / 2;

            int barY = config.positionBottom ? screenHeight - (player.isSpectator() ? 15 : (player.isCreative() ? 45 : (player.getArmor() > 0 || player.getAir() < player.getMaxAir() ? 70 : 55))) : 25;
            int textY = barY - 12;

            int totalTextWidth = 0;
            if (config.showDayCounter) totalTextWidth += 13 + client.textRenderer.getWidth(dayText);
            if (config.showDayCounter && config.showClock) totalTextWidth += 15;
            if (config.showClock) totalTextWidth += 13 + client.textRenderer.getWidth(timeText);

            int currentX = (screenWidth - totalTextWidth) / 2;

            if (config.showDayCounter) {
                boolean isMilestone = days > 0 && (days == 50 || days == 365 || days % 100 == 0);
                int textColor = 0xFFFFFFFF;

                if (isMilestone) {
                    float hue = (System.currentTimeMillis() % 4000L) / 4000f;
                    textColor = MathHelper.hsvToRgb(hue, 0.7f, 1.0f) | 0xFF000000;

                    if (lastCelebratedDay != days) {
                        lastCelebratedDay = days;
                        player.playSound(SoundEvents.ENTITY_PLAYER_LEVELUP, 0.6f, 1.0f);
                    }
                } else if (lastCelebratedDay == days) {
                    lastCelebratedDay = -1;
                }

                // RENDERING ICONE E CUORI
                if (isMilestone && lastGameMode == GameMode.SURVIVAL) {
                    // Nessuna ombra per gli ItemStack, in pieno stile Vanilla
                    ItemStack celebIcon = (days >= 500) ? new ItemStack(Items.TOTEM_OF_UNDYING) : new ItemStack(Items.GOLDEN_APPLE);
                    drawContext.getMatrices().pushMatrix();
                    drawContext.getMatrices().translate((float) currentX, (float) (textY - 2));
                    drawContext.getMatrices().scale(0.6f, 0.6f);
                    drawContext.drawItem(celebIcon, 0, 0);
                    drawContext.getMatrices().popMatrix();

                } else if (lastGameMode == GameMode.SURVIVAL) {
                    // L'ombra per il cuore 2D continua a funzionare perfettamente perché supportata nativamente (0x80000000 = nero trasparente)
                    Identifier heartTex = isHardcore ? HARDCORE_HEART_FULL : HEART_FULL;
                    drawContext.drawGuiTexture(RenderPipelines.GUI_TEXTURED, heartTex, currentX + 1, textY, 9, 9, 0x80000000);
                    drawContext.drawGuiTexture(RenderPipelines.GUI_TEXTURED, heartTex, currentX, textY - 1, 9, 9);
                } else if (!cachedIconItem.isEmpty()) {
                    // Nessuna ombra finta per icone gamemode (Spada/Occhio)
                    drawContext.getMatrices().pushMatrix();
                    drawContext.getMatrices().translate((float) currentX, (float) (textY - 1));
                    drawContext.getMatrices().scale(0.6f, 0.6f);
                    drawContext.drawItem(cachedIconItem, 0, 0);
                    drawContext.getMatrices().popMatrix();
                }

                currentX += 13;
                drawContext.drawText(client.textRenderer, dayText, currentX, textY, textColor, true);
                currentX += client.textRenderer.getWidth(dayText) + 15;
            }

            // Orologio
            if (config.showClock) {
                drawContext.getMatrices().pushMatrix();
                drawContext.getMatrices().translate((float) currentX, (float) (textY - 1));
                drawContext.getMatrices().scale(0.6f, 0.6f);
                drawContext.drawItem(new ItemStack(Items.CLOCK), 0, 0);
                drawContext.getMatrices().popMatrix();

                currentX += 13;
                drawContext.drawText(client.textRenderer, timeText, currentX, textY, 0xFFFFFFFF, true);
            }

            // Bossbar
            if (config.showBossbar) {
                drawContext.drawGuiTexture(RenderPipelines.GUI_TEXTURED, BOSS_BAR_BACKGROUND, barX, barY, barWidth, 5);
                long timeSinceMidnight = (timeOfDay + 6000L) % 24000L;
                float progress = (float) timeSinceMidnight / 24000f;
                int currentProgressWidth = (int) (barWidth * progress);

                if (currentProgressWidth > 0) {
                    float[] rgb = getSmoothColor(timeSinceMidnight);
                    int colorInt = (255 << 24) | ((int)(rgb[0]*255) << 16) | ((int)(rgb[1]*255) << 8) | (int)(rgb[2]*255);
                    drawContext.drawGuiTexture(RenderPipelines.GUI_TEXTURED, BAR_WHITE, 182, 5, 0, 0, barX, barY, currentProgressWidth, 5, colorInt);
                }
            }
        });
    }

    private float[] getSmoothColor(long ticks) {
        float[] yellow = {1.0f, 0.9f, 0.2f}, blue = {0.2f, 0.6f, 1.0f}, pink = {1.0f, 0.3f, 0.5f}, purple = {0.4f, 0.2f, 0.8f};
        if (ticks < 6000) return lerpColor(purple, yellow, ticks / 6000f);
        if (ticks < 12000) return lerpColor(yellow, blue, (ticks - 6000) / 6000f);
        if (ticks < 18000) return lerpColor(blue, pink, (ticks - 12000) / 6000f);
        return lerpColor(pink, purple, (ticks - 18000) / 6000f);
    }

    private float[] lerpColor(float[] c1, float[] c2, float t) {
        return new float[]{ c1[0] + (c2[0] - c1[0]) * t, c1[1] + (c2[1] - c1[1]) * t, c1[2] + (c2[2] - c1[2]) * t };
    }
}