CREATE TABLE members (
    id INT GENERATED ALWAYS AS IDENTITY,
    user_id INT NOT NULL,
    group_id INT NOT NULL,
    urole INT NOT NULL,
    CONSTRAINT unique_user_group_id UNIQUE(user_id, group_id),
    CONSTRAINT members_pkey PRIMARY KEY(id),
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_group FOREIGN KEY(group_id) REFERENCES sgroups(id) ON DELETE CASCADE
);
