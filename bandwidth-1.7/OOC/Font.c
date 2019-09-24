/*============================================================================
  Font, an object-oriented C font base class.
  Copyright (C) 2019 by Zack T Smith.

  Object-Oriented C is free software: you can redistribute it and/or modify
  it under the terms of the GNU Lesser General Public License as published
  by the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.
 
  Object-Oriented C is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU Lesser General Public License for more details.
 
  You should have received a copy of the GNU Lesser General Public License
  along with this software.  If not, see <http://www.gnu.org/licenses/>.

  The author may be reached at 1@zsmith.co.
 *===========================================================================*/

#include <stdlib.h>

#include "Font.h"
#include "FontBuiltin.h"

FontClass* FontClass_prepare ();

FontClass *_FontClass = NULL;

static void Font_destroy (Font *self)
{
        DEBUG_DESTROY;

	if (!self)
		return;
	verifyCorrectClass(self,Font);

        if (self->_bitmapBuffer) {
                free (self->_bitmapBuffer);
        } else {
                for (int i=0; i < 256; i++)
                        free (self->_bitmaps[i]);
        }

	clearObjectSelf;

	// NOTE: The releaser frees self.
}

static void Font_describe (Font* self, FILE *outputFile) 
{ 
	if (!self)
		return;
	verifyCorrectClass(self,Font);

	if (!outputFile)
		outputFile = stdout;

	fprintf (outputFile, "%s", self->is_a->className);
}

Font *Font_new () 
{
	Font *self = allocate(Font);
	Font_init (self);
	return self;
}

Font* Font_newWith (const char* name, int size, bool bold, bool italic)
{
	if (!_FontClass)
		FontClass_prepare ();
	Font *self = Font_new ();
	
	return self;
}

void* Font_bitmapForCharacter (Font *self, wchar_t characterCode, 
				unsigned* width, unsigned* bytesPerRow, 
				unsigned* bitsWide, unsigned* bitsHigh, 
				int* xoffset, int* descent)  
{
	verifyCorrectClass(self,Font);

	if (characterCode < self->_firstCharacter || characterCode > self->_lastCharacter)
		return NULL;

	// NOTE: Not supporting Unicode yet.
	if (characterCode > 255)
		return NULL;

	int index = characterCode - self->_firstCharacter;
	*width = self->_widths [index];
	*bytesPerRow = self->_bytesPerRow [index];
	*bitsWide = self->_bitsWide [index];
	*bitsHigh = self->_bitsHigh [index];
	*xoffset = self->_xoffsets [index];
	*descent = self->_descents [index];

	void *bitmaps = self->_bitmaps[index];
	return bitmaps;
}

void Font_sizeOfString (Font *self, String *str, int* w, int* a, int* d)      
{
	verifyCorrectClass(self,Font);
	verifyCorrectClasses(str,String,MutableString);

	int totalWidth = 0;
	wchar_t ch;

	int i = 0;
	while ((ch = $(str, characterAt, i++))) {
		if (ch == ' ') {
			totalWidth += self->_spaceWidth;
			continue;
		}

		int index = ch - self->_firstCharacter;
		if (index >= 0 && index < self->_totalCharacters) {
			totalWidth += self->_widths [index]; 
			totalWidth += self->_xoffsets [index];
			totalWidth += kDefaultIntercharacterSpace;
		}
	}

	*w = totalWidth;
	*a = self->_ascent;
	*d = self->_descent;
}

int Font_stringWidth (Font *self, String *string)
{
	verifyCorrectClass(self,Font);
	verifyCorrectClasses(string,String,MutableString);

	if (!string)
		return 0; // XX

	int width, ascent, descent;
	Font_sizeOfString (self, string, &width, &ascent, &descent);
	return width;
}

float Font_pointSize (Font *self)  
{
	return self->_pointSize;
}

short Font_ascent (Font *self)     
{
	verifyCorrectClass(self,Font);
	return self->_ascent;
}

short Font_descent (Font *self)    
{
	verifyCorrectClass(self,Font);
	return self->_descent;
}

short Font_spaceWidth (Font *self) 
{
	verifyCorrectClass(self,Font);
	return self->_spaceWidth;
}

short Font_height (Font *self)     
{
	verifyCorrectClass(self,Font);
	return self->_height;
}

wchar_t Font_firstCharacter (Font *self)   
{
	verifyCorrectClass(self,Font);
        return self->_firstCharacter;
}

wchar_t Font_lastCharacter (Font *self)    
{
	verifyCorrectClass(self,Font);
        return self->_lastCharacter;
}

long Font_totalCharacters (Font *self)
{
	verifyCorrectClass(self,Font);
        return self->_lastCharacter - self->_firstCharacter;
}

FontClass* FontClass_prepare ()
{
	PREPARE_CLASS_STRUCT(Font,Object)

	_FontClass->describe = Font_describe;
        _FontClass->destroy = Font_destroy;

	SET_METHOD_POINTER(Font,pointSize);
	SET_METHOD_POINTER(Font,stringWidth);
	SET_METHOD_POINTER(Font,totalCharacters);
	SET_METHOD_POINTER(Font,ascent);
	SET_METHOD_POINTER(Font,descent);
	SET_METHOD_POINTER(Font,height);
	SET_METHOD_POINTER(Font,spaceWidth);
	SET_METHOD_POINTER(Font,sizeOfString);
	SET_METHOD_POINTER(Font,bitmapForCharacter);
	SET_METHOD_POINTER(Font,firstCharacter);
	SET_METHOD_POINTER(Font,lastCharacter);
	
        VALIDATE_CLASS_STRUCT(_FontClass);
	return _FontClass;
}

Font* Font_init (Font *self)
{
	if (!_FontClass)
		FontClass_prepare ();

	memset (self, 0, sizeof(Font));
	Object_init ((Object*) self);

	self->is_a = _FontClass;
	
	self->_isMonochrome = true;
	self->_name = NULL;
	self->_pointSize = 0;
	self->_height = 0;
	self->_firstCharacter = 0;
	self->_lastCharacter = 0;
	self->_ascent = 0;
	self->_descent = 0;
	self->_spaceWidth = 0;
	self->_italic = false;
	self->_bold = false;
	self->_weight = 0;
	self->_isFixedWidth = false;
	self->_fixedWidth = 0;
	self->_bitmapBuffer = NULL;
	self->_family[0] = 0;
	self->_fullName[0] = 0;
//	self->_rowUnit = RowUnitByte;

	for (int i=0; i < 256; i++) {
		self->_widths[i] = 0;
		self->_xoffsets[i] = 0;
		self->_bitsHigh[i] = 0;
		self->_bitsWide[i] = 0;
		self->_descents[i] = 0;
		self->_bitmaps[i] = 0;
	}

	return self;
}

