package com.roy.kankan.convert;

import com.roy.kankan.dto.CardDetailDto;
import com.roy.kankan.entity.CardEntity;
import org.mapstruct.Mapper;
import org.mapstruct.factory.Mappers;

@Mapper
public interface CardConvert {
  CardConvert INSTANCE = Mappers.getMapper(CardConvert.class);

  CardDetailDto fromEntity(CardEntity card);
}
