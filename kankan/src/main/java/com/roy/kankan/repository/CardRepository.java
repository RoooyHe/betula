package com.roy.kankan.repository;

import com.roy.kankan.entity.CardEntity;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.JpaSpecificationExecutor;
import org.springframework.stereotype.Repository;

@Repository
public interface CardRepository
    extends JpaRepository<CardEntity, Long>, JpaSpecificationExecutor<CardEntity> {}
