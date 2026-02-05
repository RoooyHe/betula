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


}