package com.juniperparsnips.synthstone.command;

import com.juniperparsnips.synthstone.Synthstone;
import com.juniperparsnips.synthstone.util.IWorldUpdateSuppressor;
import com.mojang.brigadier.Command;
import com.mojang.brigadier.CommandDispatcher;
import com.mojang.brigadier.builder.LiteralArgumentBuilder;
import com.mojang.brigadier.context.CommandContext;
import com.mojang.brigadier.exceptions.CommandSyntaxException;
import com.mojang.brigadier.exceptions.SimpleCommandExceptionType;
import net.minecraft.server.command.CommandManager;
import net.minecraft.server.command.ServerCommandSource;
import net.minecraft.structure.Structure;
import net.minecraft.structure.StructureManager;
import net.minecraft.structure.StructurePlacementData;
import net.minecraft.text.TranslatableText;
import net.minecraft.util.*;
import net.minecraft.util.math.BlockPos;

import java.util.Optional;
import java.util.Random;

import static com.juniperparsnips.synthstone.command.arguments.BlockMirrorArgumentType.blockMirror;
import static com.juniperparsnips.synthstone.command.arguments.BlockMirrorArgumentType.getBlockMirror;
import static com.juniperparsnips.synthstone.command.arguments.BlockRotationArgumentType.blockRotation;
import static com.juniperparsnips.synthstone.command.arguments.BlockRotationArgumentType.getBlockRotation;
import static com.mojang.brigadier.arguments.BoolArgumentType.bool;
import static com.mojang.brigadier.arguments.BoolArgumentType.getBool;
import static net.minecraft.command.argument.BlockPosArgumentType.blockPos;
import static net.minecraft.command.argument.BlockPosArgumentType.getBlockPos;
import static net.minecraft.command.argument.IdentifierArgumentType.getIdentifier;
import static net.minecraft.command.argument.IdentifierArgumentType.identifier;
import static net.minecraft.server.command.CommandManager.argument;

public class PlaceStructureCommand {

    public static void register(CommandDispatcher<ServerCommandSource> dispatcher) {
        LiteralArgumentBuilder<ServerCommandSource> argumentBuilder = CommandManager.literal("placestructure")
                .requires((source) -> source.hasPermissionLevel(2))
                .then(argument("pos", blockPos()).then(argument("structure", identifier())
                        .executes((c) -> placeStructure(c, BlockMirror.NONE, BlockRotation.NONE, false))
                .then(argument("suppressUpdates", bool())
                        .executes((c) -> placeStructure(c, BlockMirror.NONE, BlockRotation.NONE, getBool(c, "suppressUpdates")))
                .then(argument("mirror", blockMirror())
                .then(argument("rotation", blockRotation())
                        .executes((c) -> placeStructure(c, getBlockMirror(c, "mirror"), getBlockRotation(c, "rotation"), getBool(c, "suppressUpdates"))))))));
        dispatcher.register(argumentBuilder);
    }

    private static int placeStructure(
            CommandContext<ServerCommandSource> c,
            BlockMirror mirror,
            BlockRotation rotation,
            boolean suppressUpdates
    ) throws CommandSyntaxException {
        StructureManager sm = c.getSource().getServer().getStructureManager();
        Optional<Structure> optional;
        try {
            Identifier id = getIdentifier(c, "structure");
            optional = sm.getStructure(id);
        } catch (InvalidIdentifierException e) {
            Synthstone.LOGGER.error("could not get structure");
            throw new SimpleCommandExceptionType(new TranslatableText("synthstone.commands.placeStructure.invalid.id")).create();
        }

        ((IWorldUpdateSuppressor) c.getSource().getWorld()).setShouldPreventBlockUpdates(suppressUpdates);

        StructurePlacementData placementData = new StructurePlacementData().setUpdateNeighbors(suppressUpdates);
        placementData.setMirror(mirror);
        placementData.setRotation(rotation);

        BlockPos pos = getBlockPos(c, "pos");
        Structure structure = optional.get();
        structure.place(c.getSource().getWorld(), pos, pos, placementData, new Random(Util.getMeasuringTimeMs()), 3);

        ((IWorldUpdateSuppressor) c.getSource().getWorld()).setShouldPreventBlockUpdates(false);

        return Command.SINGLE_SUCCESS;
    }

}
