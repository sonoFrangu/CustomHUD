package org.frangu.customhud.fabric.client;

import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents;
import net.fabricmc.fabric.api.client.keymapping.v1.KeyMappingHelper;
import net.fabricmc.fabric.api.client.rendering.v1.hud.HudElementRegistry;
import net.fabricmc.fabric.api.client.rendering.v1.hud.VanillaHudElements;
import net.fabricmc.loader.api.FabricLoader;
import net.minecraft.resources.Identifier;
import org.frangu.customhud.client.CustomhudClient;

public class CustomhudFabricClient implements ClientModInitializer {
    @Override
    public void onInitializeClient() {
        CustomhudClient.init(FabricLoader.getInstance().getConfigDir());
        KeyMappingHelper.registerKeyMapping(CustomhudClient.configKeyMapping);

        ClientTickEvents.END_CLIENT_TICK.register(CustomhudClient::onEndClientTick);
        HudElementRegistry.attachElementAfter(
                VanillaHudElements.BOSS_BAR,
                Identifier.fromNamespaceAndPath("customhud", "hud"),
                CustomhudClient::extractHud
        );
    }
}
