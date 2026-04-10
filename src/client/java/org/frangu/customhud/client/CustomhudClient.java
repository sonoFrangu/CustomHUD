package org.frangu.customhud.client;

import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.rendering.v1.HudRenderCallback;
import net.minecraft.client.MinecraftClient;
import net.minecraft.client.network.ClientPlayerEntity;
import net.minecraft.item.ItemStack;
import net.minecraft.item.Items;
import net.minecraft.text.Text;
import net.minecraft.util.Identifier;
import net.minecraft.world.GameMode;
import net.minecraft.client.gl.RenderPipelines;

public class CustomhudClient implements ClientModInitializer {

    private static final Identifier HEART_FULL = Identifier.of("minecraft", "hud/heart/full");
    private static final Identifier HARDCORE_HEART_FULL = Identifier.of("minecraft", "hud/heart/hardcore_full");
    private static final Identifier BOSS_BAR_BACKGROUND = Identifier.of("minecraft", "boss_bar/blue_background");
    private static final Identifier BAR_WHITE = Identifier.of("minecraft", "boss_bar/white_progress");

    private GameMode lastGameMode = null;
    private ItemStack cachedIconItem = ItemStack.EMPTY;

    @Override
    public void onInitializeClient() {
        HudRenderCallback.EVENT.register((drawContext, tickCounter) -> {
            MinecraftClient client = MinecraftClient.getInstance();
            ClientPlayerEntity player = client.player;

            // 1. Controlli di visibilità (F1 o F3)
            if (player == null || client.world == null) return;
            if (client.options.hudHidden || client.getDebugHud().shouldShowDebugHud()) return;

            // 2. Logica modalità di gioco (cache icona)
            GameMode currentGameMode = client.interactionManager.getCurrentGameMode();
            boolean isHardcore = client.world.getLevelProperties().isHardcore();

            if (currentGameMode != lastGameMode) {
                lastGameMode = currentGameMode;
                if (currentGameMode == GameMode.CREATIVE) cachedIconItem = new ItemStack(Items.GRASS_BLOCK);
                else if (currentGameMode == GameMode.ADVENTURE) cachedIconItem = new ItemStack(Items.IRON_SWORD);
                else if (currentGameMode == GameMode.SPECTATOR) cachedIconItem = new ItemStack(Items.ENDER_EYE);
                else cachedIconItem = ItemStack.EMPTY;
            }

            // 3. Calcolo del tempo in-game
            long totalTime = client.world.getTimeOfDay();
            long timeOfDay = totalTime % 24000L;
            long hours = (timeOfDay / 1000L + 6) % 24;
            long minutes = (timeOfDay % 1000L) * 60 / 1000L;
            long days = totalTime / 24000L;

            // 4. Calcolo coordinate e dimensioni a schermo
            int screenWidth = client.getWindow().getScaledWidth();
            int barWidth = 182;
            int barX = (screenWidth - barWidth) / 2;
            int barY = 25;
            int textY = barY - 12;

            Text dayText = Text.translatable("hud.customhud.day", days);
            String timeText = String.format("%02d:%02d", hours, minutes);

            int totalTextWidth = 13 + client.textRenderer.getWidth(dayText) + 15 + 13 + client.textRenderer.getWidth(timeText);
            int currentX = (screenWidth - totalTextWidth) / 2;

            // 5. Rendering testi e icone
            // Icona Gamemode
            if (lastGameMode == GameMode.SURVIVAL) {
                drawContext.drawGuiTexture(RenderPipelines.GUI_TEXTURED, isHardcore ? HARDCORE_HEART_FULL : HEART_FULL, currentX, textY - 1, 9, 9);
            } else if (!cachedIconItem.isEmpty()) {
                drawContext.getMatrices().pushMatrix();
                drawContext.getMatrices().translate((float) currentX, (float) (textY - 1));
                drawContext.getMatrices().scale(0.6f, 0.6f);
                drawContext.drawItem(cachedIconItem, 0, 0);
                drawContext.getMatrices().popMatrix();
            }
            currentX += 13;

            // Testo Giorno
            drawContext.drawText(client.textRenderer, dayText, currentX, textY, 0xFFFFFFFF, true);
            currentX += client.textRenderer.getWidth(dayText) + 15;

            // Icona Orologio
            drawContext.getMatrices().pushMatrix();
            drawContext.getMatrices().translate((float) currentX, (float) (textY - 1));
            drawContext.getMatrices().scale(0.6f, 0.6f);
            drawContext.drawItem(new ItemStack(Items.CLOCK), 0, 0);
            drawContext.getMatrices().popMatrix();
            currentX += 13;

            // Testo Ora
            drawContext.drawText(client.textRenderer, timeText, currentX, textY, 0xFFFFFFFF, true);

            // 6. Rendering bossbar fluida (Sfondo e Progresso)
            drawContext.drawGuiTexture(RenderPipelines.GUI_TEXTURED, BOSS_BAR_BACKGROUND, barX, barY, barWidth, 5);

            long timeSinceMidnight = (timeOfDay + 6000L) % 24000L;
            float progress = (float) timeSinceMidnight / 24000f;
            int currentProgressWidth = (int) (barWidth * progress);

            if (currentProgressWidth > 0) {
                float[] rgb = getSmoothColor(timeSinceMidnight);
                int r = (int) (rgb[0] * 255);
                int g = (int) (rgb[1] * 255);
                int b = (int) (rgb[2] * 255);
                int colorInt = (255 << 24) | (r << 16) | (g << 8) | b; // Formato ARGB

                // Disegna la texture bianca applicando il filtro colore (colorInt)
                drawContext.drawGuiTexture(RenderPipelines.GUI_TEXTURED, BAR_WHITE, 182, 5, 0, 0, barX, barY, currentProgressWidth, 5, colorInt);
            }
        });
    }

    // Calcola l'interpolazione dei colori in base alle fasi della giornata
    private float[] getSmoothColor(long ticks) {
        float[] yellow = {1.0f, 0.9f, 0.2f};
        float[] blue   = {0.2f, 0.6f, 1.0f};
        float[] pink   = {1.0f, 0.3f, 0.5f};
        float[] purple = {0.4f, 0.2f, 0.8f};

        float t;
        if (ticks < 6000) { t = ticks / 6000f; return lerpColor(purple, yellow, t); }
        else if (ticks < 12000) { t = (ticks - 6000) / 6000f; return lerpColor(yellow, blue, t); }
        else if (ticks < 18000) { t = (ticks - 12000) / 6000f; return lerpColor(blue, pink, t); }
        else { t = (ticks - 18000) / 6000f; return lerpColor(pink, purple, t); }
    }

    // Interpolazione lineare tra due colori RGB (LERP)
    private float[] lerpColor(float[] c1, float[] c2, float t) {
        return new float[]{
                c1[0] + (c2[0] - c1[0]) * t,
                c1[1] + (c2[1] - c1[1]) * t,
                c1[2] + (c2[2] - c1[2]) * t
        };
    }
}