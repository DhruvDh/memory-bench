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

#ifndef _OOC_MUTABLESTRING_H
#define _OOC_MUTABLESTRING_H

#include <stdint.h>
#include <string.h>
#include <wchar.h>

#include "Object.h"
#include "String.h"

#define DECLARE_MUTABLESTRING_POLYMORPHIC_METHODS(TYPE_POINTER) \
	int (*setASCII) (TYPE_POINTER, const char*); \
	int (*setWide) (TYPE_POINTER, const wchar_t*); \
	int (*set) (TYPE_POINTER, TYPE_POINTER); \
	int (*appendASCII) (TYPE_POINTER, const char *); \
	int (*appendWide) (TYPE_POINTER, const wchar_t *string); \
	int (*append) (TYPE_POINTER, TYPE_POINTER); \
	int (*appendCharacter) (TYPE_POINTER, wchar_t); \
	int (*truncateAt) (TYPE_POINTER, int); 

struct mutablestring;

typedef struct mutablestringclass {
	DECLARE_OBJECT_CLASS_VARS
	DECLARE_OBJECT_POLYMORPHIC_METHODS(struct mutablestring*)
	DECLARE_STRING_POLYMORPHIC_METHODS(struct mutablestring*)
	DECLARE_MUTABLESTRING_POLYMORPHIC_METHODS(struct mutablestring*)
} MutableStringClass;

extern MutableStringClass *_MutableStringClass;

#define DECLARE_MUTABLESTRING_INSTANCE_VARS(TYPE_POINTER) \
	int _allocatedSize; 

typedef struct mutablestring {
	MutableStringClass *is_a;
	DECLARE_OBJECT_INSTANCE_VARS(struct mutablestring*)
	DECLARE_STRING_INSTANCE_VARS(struct mutablestring*)
	DECLARE_MUTABLESTRING_INSTANCE_VARS(struct mutablestring*)
} MutableString;

extern MutableString* MutableString_init (MutableString* object);
extern MutableString* MutableString_newWithCString (const char*);
extern MutableStringClass* MutableStringClass_prepare ();
extern MutableString* MutableString_initWithCString (MutableString* self, const char *str);

#endif

