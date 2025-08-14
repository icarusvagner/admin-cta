-- Add migration script here

-- ENUM for optional flag
CREATE TYPE optional_flag AS ENUM ('Yes', 'No');

-- 1. LOCATIONS
CREATE TABLE tbl_location (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    city VARCHAR(255),
    province VARCHAR(255),
    category VARCHAR(100), -- e.g. Landmark, Church, Beach, Adventure
    description TEXT
);

-- 2. ITINERARY DAYS
CREATE TABLE tbl_itinerary_day (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,  -- e.g., 'Day 1 • CEBU CITY TOUR'
    description TEXT
);

-- 3. ITINERARY LOCATIONS (M:N)
CREATE TABLE tbl_itinerary_day_locations (
    id BIGSERIAL PRIMARY KEY,
    itinerary_day_id BIGINT NOT NULL REFERENCES tbl_itinerary_day(id) ON DELETE CASCADE,
    location_id BIGINT NOT NULL REFERENCES tbl_location(id) ON DELETE CASCADE,
    optional optional_flag DEFAULT 'No'
);

-- 4. PACKAGES
CREATE TABLE tbl_package (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,   -- e.g., PACKAGE 1
    description TEXT,             -- e.g., CITY TOUR + MOALBOAL & BADIAN + OSLOB + BOHOL COUNTRY SIDE TOUR
    duration_days INT NOT NULL
);

-- 5. PACKAGE -> ITINERARY DAYS (M:N)
CREATE TABLE tbl_package_itinerary (
    id BIGSERIAL PRIMARY KEY,
    package_id BIGINT NOT NULL REFERENCES tbl_package(id) ON DELETE CASCADE,
    itinerary_day_id BIGINT NOT NULL REFERENCES tbl_itinerary_day(id) ON DELETE CASCADE,
    day_order SMALLINT NOT NULL, -- e.g., 1 for Day 1, 2 for Day 2, etc.

    -- Unique constraint: no duplicate itinerary_day for same package
    CONSTRAINT uq_package_itinerary UNIQUE (package_id, itinerary_day_id)
);

-- 6. PACKAGE PRICING TIERS
CREATE TABLE tbl_package_pricing (
    id BIGSERIAL PRIMARY KEY,
    package_id BIGINT NOT NULL REFERENCES tbl_package(id) ON DELETE CASCADE,
    pax_min INT NOT NULL,
    pax_max INT,
    price_per_pax DECIMAL(10,2) NOT NULL,

    -- Unique constraint: no duplicate package_id for same package price
    CONSTRAINT uq_package_id UNIQUE (package_id)
);

-- 7. OPTIONAL ACTIVITIES
CREATE TABLE tbl_optional_activity (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    surcharge_amount DECIMAL(10,2) NOT NULL,
    unit VARCHAR(50), -- e.g., 'per pax', 'per hour'
    description TEXT
);

-- 8. ITINERARY DAY OPTIONAL ACTIVITIES (M:N)
CREATE TABLE tbl_itinerary_day_optional_activity (
    id BIGSERIAL PRIMARY KEY,
    itinerary_day_id BIGINT NOT NULL REFERENCES tbl_itinerary_day(id) ON DELETE CASCADE,
    optional_activity_id BIGINT NOT NULL REFERENCES tbl_optional_activity(id) ON DELETE CASCADE,

    CONSTRAINT uq_itinerary_optional_activity UNIQUE (itinerary_day_id, optional_activity_id)
);

-- 9. PACKAGE INCLUSIONS
CREATE TABLE tbl_inclusion (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE tbl_package_inclusion (
    id BIGSERIAL PRIMARY KEY,
    package_id BIGINT NOT NULL REFERENCES tbl_package(id) ON DELETE CASCADE,
    inclusion_id BIGINT NOT NULL REFERENCES tbl_inclusion(id) ON DELETE CASCADE,

    -- Prevent duplicates
    CONSTRAINT uq_package_inclusion UNIQUE (package_id, inclusion_id)
);

-- 10. GUESTS
CREATE TABLE tbl_guest (
    id BIGSERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    phone_number VARCHAR(50),
    country VARCHAR(100) DEFAULT 'Philippines',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 11. BOOKINGS
CREATE TYPE booking_status AS ENUM ('Pending', 'Confirmed', 'Cancelled', 'Completed');

CREATE TABLE tbl_booking (
    id BIGSERIAL PRIMARY KEY,
    guest_id BIGINT NOT NULL REFERENCES tbl_guest(id) ON DELETE CASCADE,
    package_id BIGINT NOT NULL REFERENCES tbl_package(id) ON DELETE CASCADE,
    pax_count INT NOT NULL,
    booking_date DATE NOT NULL,          -- Date guest made the booking
    start_date DATE NOT NULL,            -- Date tour starts
    end_date DATE NOT NULL,              -- Date tour ends
    status booking_status DEFAULT 'Pending',
    total_price DECIMAL(10,2) NOT NULL,
    payment_status VARCHAR(50) DEFAULT 'Unpaid',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 12. REVIEWS
CREATE TABLE tbl_review (
    id BIGSERIAL PRIMARY KEY,
    booking_id BIGINT NOT NULL REFERENCES tbl_booking(id) ON DELETE CASCADE,
    guest_id BIGINT NOT NULL REFERENCES tbl_guest(id) ON DELETE CASCADE,
    rating SMALLINT CHECK (rating BETWEEN 1 AND 5),
    review_text TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

