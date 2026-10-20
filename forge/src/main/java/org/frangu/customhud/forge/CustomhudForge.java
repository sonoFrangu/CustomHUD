package org.frangu.customhud.forge;

import net.minecraft.client.Minecraft;
import net.minecraft.resources.Identifier;
import net.minecraftforge.client.ConfigScreenHandler;
import net.minecraftforge.client.event.AddGuiOverlayLayersEvent;
import net.minecraftforge.client.event.RegisterKeyMappingsEvent;
import net.minecraftforge.client.gui.overlay.ForgeLayer;
import net.minecraftforge.client.gui.overlay.ForgeLayeredDraw;
import net.minecraftforge.event.TickEvent;
import net.minecraftforge.fml.common.Mod;
import net.minecraftforge.fml.javafmlmod.FMLJavaModLoadingContext;
import net.minecraftforge.fml.loading.FMLPaths;
import org.frangu.customhud.Customhud;
import org.frangu.customhud.client.ConfigScreen;
import org.frangu.customhud.client.CustomhudClient;

@Mod(Customhud.MOD_ID)
public final class CustomhudForge {
    public CustomhudForge(FMLJavaModLoadingContext context) {
        Customhud.init();
        CustomhudClient.init(FMLPaths.CONFIGDIR.get());

        context.registerExtensionPoint(ConfigScreenHandler.ConfigScreenFactory.class,
                () -> new ConfigScreenHandler.ConfigScreenFactory(ConfigScreen::new));

        RegisterKeyMappingsEvent.BUS.addListener(event -> event.register(CustomhudClient.CONFIG_KEY));
        TickEvent.ClientTickEvent.Post.BUS.addListener(event -> CustomhudClient.onEndClientTick(Minecraft.getInstance()));
        AddGuiOverlayLayersEvent.BUS.addListener(CustomhudForge::onAddLayers);
    }

    private static void onAddLayers(AddGuiOverlayLayersEvent event) {
        ForgeLayeredDraw root = event.getLayeredDraw();
        Identifier stack = ForgeLayeredDraw.PRE_SLEEP_STACK;

        root.addBelow(stack, CustomhudClient.HUD_LAYER_ID, ForgeLayeredDraw.BOSS_OVERLAY, CustomhudClient::extractHud);

        // Sposta le bossbar vanilla sotto l'HUD quando è in alto
        ForgeLayer bossOverlay = root.locateStack(stack).map(s -> s.getLayer(ForgeLayeredDraw.BOSS_OVERLAY)).orElse(null);
        if (bossOverlay != null) {
            root.replace(stack, ForgeLayeredDraw.BOSS_OVERLAY, (graphics, deltaTracker) -> {
                if (!CustomhudClient.shouldShiftBossBars()) {
                    bossOverlay.extract(graphics, deltaTracker);
                    return;
                }
                CustomhudClient.pushBossBarShift(graphics);
                bossOverlay.extract(graphics, deltaTracker);
                CustomhudClient.popBossBarShift(graphics);
            });
        }
    }
}
