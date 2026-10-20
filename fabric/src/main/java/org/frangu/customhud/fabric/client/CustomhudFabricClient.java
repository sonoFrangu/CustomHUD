package org.frangu.customhud.fabric.client;

import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents;
import net.fabricmc.fabric.api.client.keymapping.v1.KeyMappingHelper;
import net.fabricmc.fabric.api.client.rendering.v1.hud.HudElementRegistry;
import net.fabricmc.fabric.api.client.rendering.v1.hud.HudStatusBarHeightRegistry;
import net.fabricmc.fabric.api.client.rendering.v1.hud.VanillaHudElements;
import net.fabricmc.loader.api.FabricLoader;
import org.frangu.customhud.client.CustomhudClient;
import org.frangu.customhud.client.DayHud;

public class CustomhudFabricClient implements ClientModInitializer {
    @Override
    public void onInitializeClient() {
        CustomhudClient.init(FabricLoader.getInstance().getConfigDir());
        KeyMappingHelper.registerKeyMapping(CustomhudClient.CONFIG_KEY);
        ClientTickEvents.END_CLIENT_TICK.register(CustomhudClient::onEndClientTick);

        // Con Fabric API conosciamo l'altezza reale delle barre di stato (cuori extra, barre di altre mod)
        DayHud.statusBarHeight = () -> Math.max(
                HudStatusBarHeightRegistry.getHeight(VanillaHudElements.ARMOR_BAR),
                HudStatusBarHeightRegistry.getHeight(VanillaHudElements.AIR_BAR));

        HudElementRegistry.attachElementBefore(VanillaHudElements.BOSS_BAR,
                CustomhudClient.HUD_LAYER_ID, CustomhudClient::extractHud);

        HudElementRegistry.replaceElement(VanillaHudElements.BOSS_BAR, original -> (graphics, deltaTracker) -> {
            if (!CustomhudClient.shouldShiftBossBars()) {
                original.extractRenderState(graphics, deltaTracker);
                return;
            }
            CustomhudClient.pushBossBarShift(graphics);
            original.extractRenderState(graphics, deltaTracker);
            CustomhudClient.popBossBarShift(graphics);
        });
    }
}
