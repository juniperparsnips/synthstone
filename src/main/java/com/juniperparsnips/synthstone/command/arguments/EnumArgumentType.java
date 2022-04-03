package com.juniperparsnips.synthstone.command.arguments;

import com.mojang.brigadier.StringReader;
import com.mojang.brigadier.arguments.ArgumentType;
import com.mojang.brigadier.context.CommandContext;
import com.mojang.brigadier.exceptions.CommandSyntaxException;
import com.mojang.brigadier.exceptions.SimpleCommandExceptionType;
import com.mojang.brigadier.suggestion.Suggestions;
import com.mojang.brigadier.suggestion.SuggestionsBuilder;
import net.minecraft.command.CommandSource;
import net.minecraft.util.BlockRotation;

import java.util.Arrays;
import java.util.Collection;
import java.util.EnumSet;
import java.util.List;
import java.util.concurrent.CompletableFuture;

public abstract class EnumArgumentType<E extends Enum<E>> implements ArgumentType<E> {
    private Class<E> enumType;
    private List<String> examples;
    public SimpleCommandExceptionType unkown_enumeration_exception;

    public EnumArgumentType(Class<E> enumType, List<String> examples, SimpleCommandExceptionType unkown_enumeration_exception) {
        this.enumType = enumType;
        this.examples = examples;
        this.unkown_enumeration_exception = unkown_enumeration_exception;
    }

    public E parse(StringReader reader) throws CommandSyntaxException {
        String string = reader.readUnquotedString();
        E[] enumConstants = enumType.getEnumConstants();

        for (int i = 0; i < enumConstants.length; i++) {
            if (string.equals(examples.get(i))) {
                return enumConstants[i];
            }
        }

        throw unkown_enumeration_exception.createWithContext(reader);
    }

    public <S> CompletableFuture<Suggestions> listSuggestions(CommandContext<S> context, SuggestionsBuilder builder) {
        return CommandSource.suggestMatching(examples, builder);
    }

    public Collection<String> getExamples() {
        return examples;
    }
}
