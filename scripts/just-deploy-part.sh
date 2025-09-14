# Read secrets from .env file and deploy to Fermyon Cloud
spin-cli deploy --runtime-config-file runtime-config.toml -f spin-prod.toml \
    --variable jwt_secret="${JWT_SECRET:-${SPIN_VARIABLE_JWT_SECRET}}" \
    --variable magic_link_hmac_key="${MAGIC_LINK_HMAC_KEY:-${SPIN_VARIABLE_MAGIC_LINK_HMAC_KEY}}" \
    --variable argon2_salt="${ARGON2_SALT:-${SPIN_VARIABLE_ARGON2_SALT}}"