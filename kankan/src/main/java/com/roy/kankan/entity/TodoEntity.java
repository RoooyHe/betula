package com.roy.kankan.entity;

import jakarta.persistence.*;
import java.time.LocalDateTime;
import lombok.Getter;
import lombok.Setter;

@Entity
@Table(name = "todo")
@Getter
@Setter
public class TodoEntity {
  @Id
  @GeneratedValue(strategy = GenerationType.IDENTITY)
  private Long id;

  private String title;

  private Boolean completed;

  private LocalDateTime endTime;

  private String userId;

  private Long parentId;

  @ManyToOne(fetch = FetchType.LAZY)
  @JoinColumn(name = "card_id")
  private CardEntity card;
}
