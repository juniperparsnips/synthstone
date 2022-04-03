package com.juniperparsnips.synthstone.command.arguments;

import com.mojang.brigadier.StringReader;
import com.mojang.brigadier.arguments.ArgumentType;
import com.mojang.brigadier.context.CommandContext;
import com.mojang.brigadier.exceptions.CommandSyntaxException;
import com.mojang.brigadier.exceptions.SimpleCommandExceptionType;
import com.mojang.brigadier.suggestion.Suggestions;
import com.mojang.brigadier.suggestion.SuggestionsBuilder;
import net.minecraft.block.Block;
import net.minecraft.command.CommandSource;
import net.minecraft.server.command.ServerCommandSource;
import net.minecraft.text.TranslatableText;
import net.minecraft.util.BlockMirror;
import net.minecraft.util.BlockRotation;

import java.util.Arrays;
import java.util.Collection;
import java.util.List;
import java.util.concurrent.CompletableFuture;

public class BlockRotationArgumentType extends EnumArgumentType<BlockRotation> {
    private static final List<String> EXAMPLES = Arrays.asList("none", "cw_90", "cw_180", "ccw_90");

    public static final SimpleCommandExceptionType UNKNOWN_BLOCK_ROTATION_EXCEPTION = new SimpleCommandExceptionType(
            new TranslatableText("synthstone.commands.arguments.blockRotation.unknown")
    );

    public BlockRotationArgumentType() {
        super(BlockRotation.class, EXAMPLES, UNKNOWN_BLOCK_ROTATION_EXCEPTION);
    }

    public static BlockRotationArgumentType blockRotation() {
        return new BlockRotationArgumentType();
    }

    public static BlockRotation getBlockRotation(CommandContext<ServerCommandSource> context, String name) {
        return context.getArgument(name, BlockRotation.class);
    }
}