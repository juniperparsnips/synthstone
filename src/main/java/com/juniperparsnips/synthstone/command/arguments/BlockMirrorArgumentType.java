package com.juniperparsnips.synthstone.command.arguments;

import com.mojang.brigadier.LiteralMessage;
import com.mojang.brigadier.StringReader;
import com.mojang.brigadier.arguments.ArgumentType;
import com.mojang.brigadier.arguments.StringArgumentType;
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

import java.util.Arrays;
import java.util.Collection;
import java.util.List;
import java.util.concurrent.CompletableFuture;

public class BlockMirrorArgumentType extends EnumArgumentType<BlockMirror> {
    private static final List<String> EXAMPLES = Arrays.asList("none", "left_right", "front_back");

    public static final SimpleCommandExceptionType UNKNOWN_BLOCK_MIRROR_EXCEPTION = new SimpleCommandExceptionType(
            new TranslatableText("synthstone.commands.arguments.blockMirror.unknown")
    );

    public BlockMirrorArgumentType() {
        super(BlockMirror.class, EXAMPLES, UNKNOWN_BLOCK_MIRROR_EXCEPTION);
    }

    public static BlockMirrorArgumentType blockMirror() {
        return new BlockMirrorArgumentType();
    }

    public static BlockMirror getBlockMirror(CommandContext<ServerCommandSource> context, String name) {
        return context.getArgument(name, BlockMirror.class);
    }
}
