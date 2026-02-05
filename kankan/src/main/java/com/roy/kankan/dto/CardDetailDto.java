package com.roy.kankan.dto;

import com.roy.kankan.entity.ActiveEntity;
import com.roy.kankan.entity.CardEntity;
import com.roy.kankan.entity.TagEntity;
import com.roy.kankan.entity.TodoEntity;
import lombok.Data;

import java.time.LocalDateTime;
import java.util.List;

@Data
public class CardDetailDto {
    private Long id;
    private String title;
    private String description;
    private Boolean status;
    private LocalDateTime endTime;
    private List<TagEntity> tags;
    private List<TodoEntity> todos;
    private List<ActiveEntity> active;

    public static CardDetailDto fromEntity(CardEntity card) {
        CardDetailDto dto = new CardDetailDto();
        dto.setId(card.getId());
        dto.setTitle(card.getTitle());
        dto.setDescription(card.getDescription());
        dto.setStatus(card.getStatus());
        dto.setEndTime(card.getEndTime());
        dto.setTags(card.getTags());
        dto.setTodos(card.getTodos());
        dto.setActive(card.getActive());
        return dto;
    }
}