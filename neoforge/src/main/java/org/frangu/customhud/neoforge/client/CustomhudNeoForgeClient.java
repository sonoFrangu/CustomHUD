package org.frangu.customhud.neoforge.client;

import net.minecraft.client.Minecraft;
import net.neoforged.api.distmarker.Dist;
import net.neoforged.bus.api.SubscribeEvent;
import net.neoforged.fml.common.EventBusSubscriber;
import net.neoforged.fml.loading.FMLPaths;
import net.neoforged.neoforge.client.event.ClientTickEvent;
import net.neoforged.neoforge.client.event.RegisterGuiLayersEvent;
import net.neoforged.neoforge.client.event.RegisterKeyMappingsEvent;
import net.neoforged.neoforge.client.event.RenderGuiLayerEvent;
import net.neoforged.neoforge.client.gui.VanillaGuiLayers;
import org.frangu.customhud.Customhud;
import org.frangu.customhud.client.CustomhudClient;

@EventBusSubscriber(modid = Customhud.MOD_ID, value = Dist.CLIENT)
public final class CustomhudNeoForgeClient {
    private static boolean bossBarsShifted;

    private CustomhudNeoForgeClient() {
    }

    @SubscribeEvent
    public static void onRegisterKeyMappings(RegisterKeyMappingsEvent event) {
        CustomhudClient.init(FMLPaths.CONFIGDIR.get());
        event.register(CustomhudClient.CONFIG_KEY);
    }

    @SubscribeEvent
    public static void onRegisterGuiLayers(RegisterGuiLayersEvent event) {
        event.registerBelow(VanillaGuiLayers.BOSS_OVERLAY, CustomhudClient.HUD_LAYER_ID, CustomhudClient::extractHud);
    }

    @SubscribeEvent
    public static void onClientTick(ClientTickEvent.Post event) {
        CustomhudClient.onEndClientTick(Minecraft.getInstance());
    }

    @SubscribeEvent
    public static void onRenderLayerPre(RenderGuiLayerEvent.Pre event) {
        if (event.getName().equals(VanillaGuiLayers.BOSS_OVERLAY) && CustomhudClient.shouldShiftBossBars()) {
            CustomhudClient.pushBossBarShift(event.getGuiGraphics());
            bossBarsShifted = true;
        }
    }

    @SubscribeEvent
    public static void onRenderLayerPost(RenderGuiLayerEvent.Post event) {
        if (bossBarsShifted && event.getName().equals(VanillaGuiLayers.BOSS_OVERLAY)) {
            CustomhudClient.popBossBarShift(event.getGuiGraphics());
            bossBarsShifted = false;
        }
    }
}
