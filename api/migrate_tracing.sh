#!/usr/bin/env bash
# Script automatizado para migrar println!/eprintln! a tracing
# Preserva líneas ya comentadas y aplica los niveles correctos

set -euo pipefail

echo "🔧 Iniciando migración automatizada a tracing..."

# Función para migrar un archivo
migrate_file() {
    local file="$1"
    echo "  📝 Migrando: $file"

    # Backup
    cp "$file" "$file.bak"

    # 1. Agregar import de tracing si no existe
    if ! grep -q "use tracing::" "$file" 2>/dev/null; then
        # Encontrar la sección de imports y agregar tracing
        sed -i '/^use /a use tracing::{info, warn, error, debug};' "$file"
    fi

    # 2. Migrar println! NO comentados según patrones
    # ERROR: 🚨, ❌, SECURITY VIOLATION, CRITICAL
    sed -i 's/^\(\s*\)println!\(.*🚨.*\)/\1\/\/ println!\2\n\1error!\2/' "$file"
    sed -i 's/^\(\s*\)println!\(.*❌.*\)/\1\/\/ println!\2\n\1error!\2/' "$file"
    sed -i 's/^\(\s*\)println!\(.*SECURITY VIOLATION.*\)/\1\/\/ println!\2\n\1error!\2/' "$file"
    sed -i 's/^\(\s*\)println!\(.*CRITICAL.*\)/\1\/\/ println!\2\n\1error!\2/' "$file"

    # WARN: ⚠️
    sed -i 's/^\(\s*\)println!\(.*⚠️.*\)/\1\/\/ println!\2\n\1warn!\2/' "$file"

    # DEBUG: 🔍 DEBUG
    sed -i 's/^\(\s*\)println!\(.*🔍 DEBUG.*\)/\1\/\/ println!\2\n\1debug!\2/' "$file"

    # INFO: ✅, resto de casos
    sed -i 's/^\(\s*\)println!\(.*✅.*\)/\1\/\/ println!\2\n\1info!\2/' "$file"

    # Migrar eprintln! NO comentados (todos a error!)
    sed -i 's/^\(\s*\)eprintln!\(.*\)/\1\/\/ eprintln!\2\n\1error!\2/' "$file"

    # Migrar println! restantes que no tengan emojis a info!
    sed -i 's/^\(\s*\)println!\([^/]*\)$/\1\/\/ println!\2\n\1info!\2/' "$file"
}

# Migrar todos los archivos .rs en src/ que NO están ya migrados
find src -name "*.rs" -type f | while read -r file; do
    # Skip si el archivo ya tiene "use tracing::" y líneas comentadas con println!
    if grep -q "use tracing::" "$file" 2>/dev/null && grep -q "// println!" "$file" 2>/dev/null; then
        echo "  ⏭️  Ya migrado: $file"
        continue
    fi

    # Skip si no tiene println! ni eprintln!
    if ! grep -q -E "(println!|eprintln!)" "$file" 2>/dev/null; then
        continue
    fi

    migrate_file "$file"
done

echo ""
echo "✅ Migración automatizada completada"
echo ""
echo "📊 Verificando resultados..."
echo "   println! activos restantes: $(grep -r "^\s*println!" src --include="*.rs" 2>/dev/null | wc -l || echo 0)"
echo "   eprintln! activos restantes: $(grep -r "^\s*eprintln!" src --include="*.rs" 2>/dev/null | wc -l || echo 0)"
echo ""
echo "🧹 Para limpiar backups: find src -name '*.rs.bak' -delete"
