/*============================================================================
  MutableString, an object-oriented C string manipulation class.
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

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <wchar.h>

#include "MutableString.h"

MutableStringClass *_MutableStringClass = NULL;

#define kDefaultMutableStringSize (32)

static void MutableString_destroy (MutableString* self)
{
        DEBUG_DESTROY;

	if (!self)
		return;
	verifyCorrectClass(self,MutableString);
	
	if (self->_characters) {
		free (self->_characters);
	}

	clearObjectSelf;
}

MutableString* MutableString_init (MutableString* self)
{
	if (!self)
		return NULL;
	
	self->is_a = _MutableStringClass;

	self->_characters = malloc (sizeof(wchar_t) * kDefaultMutableStringSize);
	self->_allocatedSize = kDefaultMutableStringSize;
	self->_length = 0;

	return self;
}

MutableString* MutableString_initWithCString (MutableString* self, const char *str)
{
	if (self) {
		self->is_a = _MutableStringClass;

		int len = str ? strlen (str) : 0;
		self->_length = len;
		self->_allocatedSize = len? len*2+1 : kDefaultMutableStringSize;
		self->_characters = malloc (sizeof(wchar_t) * self->_allocatedSize);
		if (str) {
			for (int i=0; i < len; i++)
				self->_characters[i] = str[i];
		}
	}
	return self;
}

static void reallocIfNecessary (MutableString *self, int newLength)
{
	if (newLength >= self->_allocatedSize-1) {
		int newSize = self->_allocatedSize  * 2;
		if (newLength > newSize)
			newSize = newLength * 2;
		self->_characters = realloc (self->_characters, sizeof(wchar_t) * newSize);
		self->_allocatedSize = newSize;
	}
}

int MutableString_setASCII (MutableString *self, const char* string)
{
	if (!self) 
		return -1;
	verifyCorrectClass(self,MutableString);
	if (!string || !*string) {
		self->_length = 0;
		return 0;
	}
	int len = strlen (string);
	reallocIfNecessary (self, len);
	int i;
	for (i=0; i < len; i++) {
		self->_characters[i] = string[i];
	}
	self->_characters[i] = 0;
	self->_length = len;
	return len;
}

int MutableString_setWide (MutableString *self, const wchar_t* string)
{
	if (!self) 
		return -1;
	verifyCorrectClass(self,MutableString);
	if (!string || !*string) {
		self->_length = 0;
		return 0;
	}
	int len = wcslen (string);
	reallocIfNecessary (self, len);
	wmemcpy (self->_characters, string, len);
	self->_characters[len] = 0;
	self->_length = len;
	return len;
}

int MutableString_setMutableString (MutableString *self, MutableString *string)
{
	if (!self) 
		return -1;
	verifyCorrectClass(self,MutableString);
	if (!string) {
		self->_length = 0;
		return 0;
	}
	int len = string->_length;
	reallocIfNecessary (self, len);
	wmemcpy (self->_characters, string->_characters, len);
	self->_characters[len] = 0;
	self->_length = len;
	return len;
}

int MutableString_appendASCII (MutableString *self, const char *string)
{
	if (!self)
		return -1;
	verifyCorrectClass(self,MutableString);
	if (!string || !*string)
		return self->_length;
	int len = strlen (string);
	int newLength = self->_length + len;
	reallocIfNecessary (self, newLength);
	int i=0;
	int j=self->_length;
	while (i < len) {
		self->_characters[j++] = (wchar_t) string[i++];
	}
	self->_characters[j] = 0;
	self->_length = newLength;
	return newLength;
}

int MutableString_appendWide (MutableString *self, const wchar_t *string)
{
	if (!self)
		return -1;
	verifyCorrectClass(self,MutableString);
	if (!string || !*string)
		return self->_length;
	int len = wcslen (string);
	int newLength = self->_length + len;
	reallocIfNecessary (self, newLength);
	wmemcpy (self->_characters + self->_length, string, len);
	self->_length = newLength;
	self->_characters [newLength] = 0;
	return newLength;
}

int MutableString_appendMutableString (MutableString *self, MutableString *that)
{
	if (!self)
		return -1;
	verifyCorrectClass(self,MutableString);
	if (!that)
		return self->_length;
	verifyCorrectClasses(that,String,MutableString);

	int newLength = self->_length + that->_length;
	reallocIfNecessary (self, newLength);
	wmemcpy (self->_characters + self->_length, that->_characters, that->_length);
	self->_length = newLength;
	self->_characters [newLength] = 0;
	return newLength;
}

int MutableString_appendCharacter (MutableString *self, wchar_t ch)
{
	if (!self)
		return -1;
	verifyCorrectClass(self,MutableString);
	if (!ch)
		return self->_length;
	int length = self->_length;
	reallocIfNecessary (self, 1+length);
	self->_characters [length++] = ch;
	self->_characters [length] = 0;
	self->_length = length;
	return length;
}

int MutableString_truncateAt (MutableString *self, int index)
{
	if (!self)
		return -1;
	verifyCorrectClass(self,MutableString);
	if (index < self->_length) {
		self->_characters[index] = 0;
		self->_length = index;
	}
	return self->_length;
}

MutableString* MutableString_newWithCString (const char* str)
{
	MutableString* obj = new(MutableString);
	return MutableString_initWithCString (obj, str);
}

MutableStringClass* MutableStringClass_prepare ()
{
	PREPARE_CLASS_STRUCT(MutableString,String)

	// Overridden method
        _MutableStringClass->destroy = MutableString_destroy;

	// Inherited methods
	_MutableStringClass->describe = (void*) String_describe;
        _MutableStringClass->length = (void*) String_length;
        _MutableStringClass->print = (void*) String_print;
	_MutableStringClass->characterAt = (void*) String_characterAt;

	// Additional methods
	_MutableStringClass->setASCII = MutableString_setASCII;
	_MutableStringClass->setWide = MutableString_setWide;
	_MutableStringClass->set = MutableString_setMutableString;
	_MutableStringClass->appendASCII = MutableString_appendASCII;
	_MutableStringClass->appendWide = MutableString_appendWide;
	_MutableStringClass->appendCharacter= MutableString_appendCharacter;
	_MutableStringClass->append = MutableString_appendMutableString;
	_MutableStringClass->truncateAt = MutableString_truncateAt;
	
	VALIDATE_CLASS_STRUCT(_MutableStringClass);
	return _MutableStringClass;
}

