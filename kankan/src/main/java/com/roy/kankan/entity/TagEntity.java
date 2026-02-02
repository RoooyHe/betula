package com.roy.kankan.entity;

import com.fasterxml.jackson.annotation.JsonIgnore;
import jakarta.persistence.*;
import lombok.Getter;
import lombok.Setter;
import java.util.List;

@Entity
@Table(name = "tag")
@Getter
@Setter
public class TagEntity {
  @Id
  @GeneratedValue(strategy = GenerationType.IDENTITY)
  private Long id;

  private String title;

  private String color;

  @JsonIgnore
  @ManyToMany(mappedBy = "tags")
  private List<CardEntity> cards;
}
