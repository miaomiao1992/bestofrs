ALTER TABLE public.repos ADD COLUMN IF NOT EXISTS homepage_url text;
ALTER TABLE public.repos ADD COLUMN IF NOT EXISTS avatar_url text;
