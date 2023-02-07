CREATE TABLE `config` (
  `name` varchar(64) NOT NULL,
  `value` text NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

ALTER TABLE `config`
  ADD PRIMARY KEY (`name`);
COMMIT;