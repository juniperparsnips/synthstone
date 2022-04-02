package com.juniperparsnips.synthstone.mixin;

import com.juniperparsnips.synthstone.util.IWorldUpdateSuppressor;
import net.minecraft.world.World;
import org.spongepowered.asm.mixin.Mixin;

@Mixin(World.class)
public class MixinWorld implements IWorldUpdateSuppressor
{
    private boolean preventBlockUpdates;

    @Override
    public boolean getShouldPreventBlockUpdates()
    {
        return this.preventBlockUpdates;
    }

    @Override
    public void setShouldPreventBlockUpdates(boolean preventUpdates)
    {
        this.preventBlockUpdates = preventUpdates;
    }
}