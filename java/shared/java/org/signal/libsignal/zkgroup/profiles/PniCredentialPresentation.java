//
// Copyright 2021 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

package org.signal.libsignal.zkgroup.profiles;

import org.signal.libsignal.zkgroup.InvalidInputException;
import org.signal.libsignal.zkgroup.groups.ProfileKeyCiphertext;
import org.signal.libsignal.zkgroup.groups.UuidCiphertext;
import org.signal.libsignal.zkgroup.internal.ByteArray;
import org.signal.libsignal.internal.Native;

public final class PniCredentialPresentation extends ByteArray {
  public PniCredentialPresentation(byte[] contents) throws InvalidInputException {
    super(contents);
    try {
      Native.PniCredentialPresentation_CheckValidContents(contents);
    } catch (IllegalArgumentException e) {
      throw new InvalidInputException(e.getMessage());
    }
  }

  public UuidCiphertext getAciCiphertext() {
    byte[] newContents = Native.PniCredentialPresentation_GetAciCiphertext(contents);

    try {
      return new UuidCiphertext(newContents);
    } catch (InvalidInputException e) {
      throw new AssertionError(e);
    }
  }

  public UuidCiphertext getPniCiphertext() {
    byte[] newContents = Native.PniCredentialPresentation_GetPniCiphertext(contents);

    try {
      return new UuidCiphertext(newContents);
    } catch (InvalidInputException e) {
      throw new AssertionError(e);
    }
  }

  public ProfileKeyCiphertext getProfileKeyCiphertext() {
    byte[] newContents = Native.PniCredentialPresentation_GetProfileKeyCiphertext(contents);

    try {
      return new ProfileKeyCiphertext(newContents);
    } catch (InvalidInputException e) {
      throw new AssertionError(e);
    }
  }

}
