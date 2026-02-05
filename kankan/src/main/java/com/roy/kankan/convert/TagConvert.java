package com.roy.kankan.convert;

import com.roy.kankan.command.CreateTagCommand;
import com.roy.kankan.entity.TagEntity;
import org.mapstruct.Mapper;
import org.mapstruct.Mapping;
import org.mapstruct.Mappings;
import org.mapstruct.factory.Mappers;

@Mapper
public interface TagConvert {
  TagConvert INSTANCE = Mappers.getMapper(TagConvert.class);

  @Mappings({@Mapping(target = "cards", ignore = true), @Mapping(target = "id", ignore = true)})
  TagEntity createCommandToEntity(CreateTagCommand command);
}
