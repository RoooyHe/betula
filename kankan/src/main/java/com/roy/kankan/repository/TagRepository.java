package com.roy.kankan.repository;

import com.roy.kankan.entity.TagEntity;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.JpaSpecificationExecutor;

public interface TagRepository
    extends JpaRepository<TagEntity, Long>, JpaSpecificationExecutor<TagEntity> {}
