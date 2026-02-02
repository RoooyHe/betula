package com.roy.kankan.entity;

import com.fasterxml.jackson.annotation.JsonBackReference;
import com.fasterxml.jackson.annotation.JsonIgnore;
import jakarta.persistence.*;
import java.time.LocalDateTime;
import java.util.List;
import lombok.Getter;
import lombok.Setter;

@Entity
@Table(name = "card")
@Getter
@Setter
public class CardEntity {
  @Id
  @GeneratedValue(strategy = GenerationType.IDENTITY)
  private Long id;

  private String title;

  private Boolean status;

  @ManyToMany(fetch = FetchType.EAGER)
  @JoinTable(
      name = "card_tag",
      joinColumns = @JoinColumn(name = "card_id"),
      inverseJoinColumns = @JoinColumn(name = "tag_id"))
  private List<TagEntity> tags;

  private LocalDateTime endTime;

  private String description;

  @JsonBackReference
  @ManyToOne
  @JoinColumn(name = "space_id")
  private SpaceEntity space;

  @JsonIgnore
  @OneToMany(mappedBy = "card", cascade = CascadeType.ALL)
  private List<TodoEntity> todos;

  @JsonIgnore
  @OneToMany(mappedBy = "card", cascade = CascadeType.ALL)
  private List<ActiveEntity> active;
}
