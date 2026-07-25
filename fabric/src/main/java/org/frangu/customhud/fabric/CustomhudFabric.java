package org.frangu.customhud.fabric;

import net.fabricmc.api.ModInitializer;
import org.frangu.customhud.Customhud;

public class CustomhudFabric implements ModInitializer {
    @Override
    public void onInitialize() {
        Customhud.init();
    }
}
