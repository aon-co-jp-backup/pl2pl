package com.example;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

@Service
public class UserService {
    @Autowired
    private UserRepository repo;
    private static final int MAX_AGE = 150;

    public User create(User u) {
        if (u.getName() == null || u.getName().trim().isEmpty()) {
            throw new IllegalArgumentException("name is required");
        }
        if (u.getAge() < 0 || u.getAge() > MAX_AGE) {
            throw new IllegalArgumentException("bad age: " + u.getAge());
        }
        if (repo.existsByEmail(u.getEmail())) {
            throw new IllegalStateException("email taken");
        }
        u.setActive(true);
        return repo.save(u);
    }

    public Optional<User> find(Long id) {
        return repo.findById(id);
    }

    public List<User> all() {
        return repo.findAll();
    }

    public List<User> byName(String name) {
        return repo.findByName(name);
    }

    public String grade(int score) {
        if (score >= 90) {
            return "A";
        } else if (score >= 80) {
            return "B";
        }
        String s = "C";
        for (int i = 0; i < 2; i++) {
            s = s + "!";
        }
        return s;
    }

    public int sum(List<Integer> xs) {
        int total = 0;
        for (int x : xs) {
            total += x;
        }
        return total;
    }

    public String initials(String name) {
        StringBuilder sb = new StringBuilder();
        for (String part : name.split(" ")) {
            if (!part.isEmpty()) {
                sb.append(part.substring(0, 1).toUpperCase());
            }
        }
        return sb.toString();
    }

    public boolean delete(Long id) {
        if (!repo.existsById(id)) {
            return false;
        }
        repo.deleteById(id);
        return true;
    }
}
