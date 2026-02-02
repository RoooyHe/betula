package com.roy.kankan.entity;

import com.fasterxml.jackson.annotation.JsonManagedReference;
import com.fasterxml.jackson.annotation.JsonIgnore;
import jakarta.persistence.*;
import java.util.List;
import lombok.Getter;
import lombok.Setter;

@Entity
@Table(name = "space")
@Getter
@Setter
public class SpaceEntity {
  @Id
  @GeneratedValue(strategy = GenerationType.IDENTITY)
  private Long id;

  private String title;

  private String userId;

  private Boolean canceled;

  private Integer sort;

  private String color;

  private String sortBy;

  @JsonManagedReference
  @OneToMany(mappedBy = "space", fetch = FetchType.EAGER, cascade = CascadeType.ALL)
  private List<CardEntity> cards;
}
