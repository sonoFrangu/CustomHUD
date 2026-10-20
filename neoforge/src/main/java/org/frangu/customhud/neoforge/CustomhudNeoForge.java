package org.frangu.customhud.neoforge;

import net.neoforged.api.distmarker.Dist;
import net.neoforged.bus.api.IEventBus;
import net.neoforged.fml.ModContainer;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.client.gui.IConfigScreenFactory;
import org.frangu.customhud.Customhud;
import org.frangu.customhud.client.ConfigScreen;

@Mod(value = Customhud.MOD_ID, dist = Dist.CLIENT)
public class CustomhudNeoForge {
    public CustomhudNeoForge(IEventBus modBus, ModContainer container) {
        Customhud.init();
        // Pulsante "Config" nella lista mod
        container.registerExtensionPoint(IConfigScreenFactory.class, (mod, parent) -> new ConfigScreen(parent));
    }
}
