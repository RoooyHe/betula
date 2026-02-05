package com.roy.kankan.controller;

import com.roy.kankan.dto.CardDetailDto;
import com.roy.kankan.entity.CardEntity;
import com.roy.kankan.repository.CardRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/v1/card")
@RequiredArgsConstructor
public class CardController {

  private final CardRepository cardRepository;

  @GetMapping
  public ResponseEntity<List<CardEntity>> getAllCards() {
    return ResponseEntity.ok(cardRepository.findAll());
  }

  @GetMapping("/{id}")
  public ResponseEntity<CardEntity> getCardById(@PathVariable Long id) {
    return cardRepository
        .findById(id)
        .map(ResponseEntity::ok)
        .orElse(ResponseEntity.notFound().build());
  }

  @GetMapping("/{id}/detail")
  public ResponseEntity<CardDetailDto> getCardDetailById(@PathVariable Long id) {
    return cardRepository
        .findById(id)
        .map(card -> ResponseEntity.ok(CardDetailDto.fromEntity(card)))
        .orElse(ResponseEntity.notFound().build());
  }

  @PostMapping
  public ResponseEntity<CardEntity> createCard(@RequestBody CardEntity card) {
    CardEntity savedCard = cardRepository.save(card);
    return ResponseEntity.ok(savedCard);
  }

  @PutMapping("/{id}")
  public ResponseEntity<CardEntity> updateCard(
      @PathVariable Long id, @RequestBody CardEntity cardDetails) {
    return cardRepository
        .findById(id)
        .map(
            card -> {
              card.setTitle(cardDetails.getTitle());
              card.setStatus(cardDetails.getStatus());
              card.setTags(cardDetails.getTags());
              card.setEndTime(cardDetails.getEndTime());
              card.setDescription(cardDetails.getDescription());
              card.setTodos(cardDetails.getTodos());
              card.setActive(cardDetails.getActive());
              return ResponseEntity.ok(cardRepository.save(card));
            })
        .orElse(ResponseEntity.notFound().build());
  }

  @DeleteMapping("/{id}")
  public ResponseEntity<Void> deleteCard(@PathVariable Long id) {
    if (cardRepository.existsById(id)) {
      cardRepository.deleteById(id);
      return ResponseEntity.ok().build();
    }
    return ResponseEntity.notFound().build();
  }
}
