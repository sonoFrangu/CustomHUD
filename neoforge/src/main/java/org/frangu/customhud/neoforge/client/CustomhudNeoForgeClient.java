package org.frangu.customhud.neoforge.client;

import net.neoforged.api.distmarker.Dist;
import net.neoforged.bus.api.SubscribeEvent;
import net.neoforged.fml.common.EventBusSubscriber;
import net.neoforged.fml.event.lifecycle.FMLClientSetupEvent;
import net.neoforged.neoforge.client.event.ClientTickEvent;
import net.neoforged.neoforge.client.event.RegisterKeyMappingsEvent;
import net.neoforged.neoforge.client.event.RenderGuiLayerEvent;
import net.neoforged.neoforge.client.gui.VanillaGuiLayers;
import net.neoforged.neoforge.common.NeoForge;
import org.frangu.customhud.Customhud;
import org.frangu.customhud.client.CustomhudClient;
import net.minecraft.client.Minecraft;
import net.neoforged.fml.loading.FMLPaths;

@EventBusSubscriber(modid = Customhud.MOD_ID, value = Dist.CLIENT)
public class CustomhudNeoForgeClient {

    @SubscribeEvent
    public static void onClientSetup(FMLClientSetupEvent event) {
        NeoForge.EVENT_BUS.addListener(CustomhudNeoForgeClient::onClientTick);
        NeoForge.EVENT_BUS.addListener(CustomhudNeoForgeClient::onRenderGuiLayer);
    }

    @SubscribeEvent
    public static void onRegisterKeyMappings(RegisterKeyMappingsEvent event) {
        CustomhudClient.init(FMLPaths.CONFIGDIR.get());
        event.register(CustomhudClient.configKeyMapping);
    }

    public static void onClientTick(ClientTickEvent.Post event) {
        CustomhudClient.onEndClientTick(Minecraft.getInstance());
    }

    public static void onRenderGuiLayer(RenderGuiLayerEvent.Post event) {
        // Render after hotbar or bossbar
        if (event.getName().equals(VanillaGuiLayers.BOSS_OVERLAY)) {
            CustomhudClient.extractHud(event.getGuiGraphics(), event.getPartialTick());
        }
    }
}
