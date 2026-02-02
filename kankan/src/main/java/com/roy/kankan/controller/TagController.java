package com.roy.kankan.controller;

import com.roy.kankan.command.CreateTagCommand;
import com.roy.kankan.convert.TagConvert;
import com.roy.kankan.entity.CardEntity;
import com.roy.kankan.entity.TagEntity;
import com.roy.kankan.repository.TagRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/v1/tag")
@RequiredArgsConstructor
public class TagController {

  private final TagRepository tagRepository;

  @GetMapping
  public ResponseEntity<List<TagEntity>> getAllTags() {
    return ResponseEntity.ok(tagRepository.findAll());
  }

  @GetMapping("/{id}")
  public ResponseEntity<TagEntity> getTagById(@PathVariable Long id) {
    return tagRepository
        .findById(id)
        .map(ResponseEntity::ok)
        .orElse(ResponseEntity.notFound().build());
  }

  @PostMapping
  public ResponseEntity<TagEntity> createTag(@RequestBody CreateTagCommand command) {
    TagEntity commandToEntity = TagConvert.INSTANCE.createCommandToEntity(command);
    TagEntity savedTag = tagRepository.save(commandToEntity);
    return ResponseEntity.ok(savedTag);
  }

  @PutMapping("/{id}")
  public ResponseEntity<TagEntity> updateTag(
      @PathVariable Long id, @RequestBody TagEntity tagDetails) {
    return tagRepository
        .findById(id)
        .map(
            tag -> {
              tag.setTitle(tagDetails.getTitle());
              tag.setColor(tagDetails.getColor());
              return ResponseEntity.ok(tagRepository.save(tag));
            })
        .orElse(ResponseEntity.notFound().build());
  }

  @DeleteMapping("/{id}")
  public ResponseEntity<Void> deleteTag(@PathVariable Long id) {
    if (tagRepository.existsById(id)) {
      tagRepository.deleteById(id);
      return ResponseEntity.ok().build();
    }
    return ResponseEntity.notFound().build();
  }
}
