from PIL import Image, ImageDraw, ImageFont

def create_icon(size, filename):
    # Créer image RGBA (avec alpha)
    img = Image.new('RGBA', (size, size), (255, 102, 0, 255))
    draw = ImageDraw.Draw(img)
    
    # Calculer taille police
    font_size = int(size * 0.6)
    
    # Texte blanc
    text = "MF"
    
    # Centrer le texte (approximatif sans font)
    text_width = size // 2
    text_height = size // 3
    
    x = (size - text_width) // 2
    y = (size - text_height) // 2
    
    draw.text((x, y), text, fill=(255, 255, 255, 255))
    
    # Sauvegarder en RGBA
    img.save(filename, 'PNG')
    print(f"✅ {filename} créé (RGBA)")

# Créer les icônes
create_icon(32, 'src-tauri/icons/32x32.png')
create_icon(128, 'src-tauri/icons/128x128.png')
create_icon(256, 'src-tauri/icons/128x128@2x.png')

# Copier pour .ico et .icns
import shutil
shutil.copy('src-tauri/icons/128x128.png', 'src-tauri/icons/icon.ico')
shutil.copy('src-tauri/icons/128x128.png', 'src-tauri/icons/icon.icns')

print("✅ Toutes les icônes créées en RGBA")
