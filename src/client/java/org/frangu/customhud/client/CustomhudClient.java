package org.frangu.customhud.client;

import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.rendering.v1.HudRenderCallback;
import net.minecraft.client.MinecraftClient;
import net.minecraft.client.network.ClientPlayerEntity;
import net.minecraft.item.ItemStack;
import net.minecraft.item.Items;
import net.minecraft.util.Identifier;
import net.minecraft.world.GameMode;
import net.minecraft.client.gl.RenderPipelines;

public class CustomhudClient implements ClientModInitializer {

    private static final Identifier HEART_FULL = Identifier.of("minecraft", "hud/heart/full");
    private static final Identifier HARDCORE_HEART_FULL = Identifier.of("minecraft", "hud/heart/hardcore_full");
    private static final Identifier BOSS_BAR_BACKGROUND = Identifier.of("minecraft", "boss_bar/blue_background");
    private static final Identifier BOSS_BAR_PROGRESS = Identifier.of("minecraft", "boss_bar/blue_progress");

    private GameMode lastGameMode = null;
    private ItemStack cachedIconItem = ItemStack.EMPTY;

    @Override
    public void onInitializeClient() {
        HudRenderCallback.EVENT.register((drawContext, tickCounter) -> {
            MinecraftClient client = MinecraftClient.getInstance();
            ClientPlayerEntity player = client.player;

            if (player == null || client.world == null) return;
            if (client.options.hudHidden) return;
            if (client.getDebugHud().shouldShowDebugHud()) return;

            GameMode currentGameMode = client.interactionManager.getCurrentGameMode();
            boolean isHardcore = client.world.getLevelProperties().isHardcore();

            // Aggiornamento cache (Ora l'Hardcore viene gestito solo al momento di disegnare)
            if (currentGameMode != lastGameMode) {
                lastGameMode = currentGameMode;
                if (currentGameMode == GameMode.CREATIVE) cachedIconItem = new ItemStack(Items.GRASS_BLOCK);
                else if (currentGameMode == GameMode.ADVENTURE) cachedIconItem = new ItemStack(Items.IRON_SWORD);
                else if (currentGameMode == GameMode.SPECTATOR) cachedIconItem = new ItemStack(Items.ENDER_EYE);
                else cachedIconItem = ItemStack.EMPTY;
            }

            long time = client.world.getTimeOfDay();
            long days = time / 24000L;
            long timeOfDay = time % 24000L;
            long hours = (timeOfDay / 1000L + 6) % 24;
            long minutes = (timeOfDay % 1000L) * 60 / 1000L;

            String dayText = "Day: " + days;
            String timeText = String.format("%02d:%02d", hours, minutes);

            int screenWidth = client.getWindow().getScaledWidth();
            int barWidth = 182;
            int barX = (screenWidth - barWidth) / 2;
            int barY = 25;

            int dayWidth = client.textRenderer.getWidth(dayText);
            int timeWidth = client.textRenderer.getWidth(timeText);
            int iconSize = 9;
            int space = 4;
            int gap = 15;

            int totalTextWidth = iconSize + space + dayWidth + gap + iconSize + space + timeWidth;
            int startX = (screenWidth - totalTextWidth) / 2;
            int textY = barY - 12;

            int currentX = startX;

            // DISEGNO ICONA CORRETTO
            if (lastGameMode == GameMode.SURVIVAL) {
                if (isHardcore) {
                    drawContext.drawGuiTexture(RenderPipelines.GUI_TEXTURED, HARDCORE_HEART_FULL, currentX, textY - 1, 9, 9);
                } else {
                    drawContext.drawGuiTexture(RenderPipelines.GUI_TEXTURED, HEART_FULL, currentX, textY - 1, 9, 9);
                }
            } else if (!cachedIconItem.isEmpty()) {
                drawContext.getMatrices().pushMatrix();
                drawContext.getMatrices().translate((float) currentX, (float) (textY - 1));
                drawContext.getMatrices().scale(0.6f, 0.6f);
                drawContext.drawItem(cachedIconItem, 0, 0);
                drawContext.getMatrices().popMatrix();
            }
            currentX += iconSize + space;

            drawContext.drawText(client.textRenderer, dayText, currentX, textY, 0xFFFFFFFF, true);
            currentX += dayWidth + gap;

            drawContext.getMatrices().pushMatrix();
            drawContext.getMatrices().translate((float) currentX, (float) (textY - 1));
            drawContext.getMatrices().scale(0.6f, 0.6f);
            drawContext.drawItem(new ItemStack(Items.CLOCK), 0, 0);
            drawContext.getMatrices().popMatrix();
            currentX += iconSize + space;

            drawContext.drawText(client.textRenderer, timeText, currentX, textY, 0xFFFFFFFF, true);

            drawContext.drawGuiTexture(RenderPipelines.GUI_TEXTURED, BOSS_BAR_BACKGROUND, barX, barY, barWidth, 5);

            long timeSinceMidnight = (timeOfDay + 6000L) % 24000L;
            float progress = (float) timeSinceMidnight / 24000f;
            int currentProgress = (int) (barWidth * progress);

            if (currentProgress > 0) {
                drawContext.drawGuiTexture(RenderPipelines.GUI_TEXTURED, BOSS_BAR_PROGRESS, 182, 5, 0, 0, barX, barY, currentProgress, 5);
            }
        });
    }
}