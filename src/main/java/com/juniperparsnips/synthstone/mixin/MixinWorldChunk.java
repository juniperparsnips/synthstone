package com.juniperparsnips.synthstone.mixin;

import com.juniperparsnips.synthstone.util.IWorldUpdateSuppressor;
import net.minecraft.world.World;
import net.minecraft.world.chunk.WorldChunk;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Redirect;
import org.spongepowered.asm.mixin.injection.Slice;

@Mixin(WorldChunk.class)
public abstract class MixinWorldChunk {
    @Redirect(method = "setBlockState",
            slice = @Slice(from = @At(value = "INVOKE",
                    target = "Lnet/minecraft/world/chunk/ChunkSection;getBlockState(III)" +
                            "Lnet/minecraft/block/BlockState;")),
            at = @At(value = "FIELD", target = "Lnet/minecraft/world/World;isClient:Z", ordinal = 0))
    private boolean redirectIsRemote(World world) {
        return ((IWorldUpdateSuppressor) world).getShouldPreventBlockUpdates() || world.isClient;
    }
}
